#include <stdio.h>
#include <stdlib.h>
#include <mach/mach.h>
#include <sys/sysctl.h>
#include <ifaddrs.h>
#include <net/if.h>
#include <sys/mount.h>
#include <math.h>
#include <unistd.h>
#include <DiskArbitration/DiskArbitration.h>
#include <CoreFoundation/CoreFoundation.h>
#include "../config/config.h"

typedef struct {
    unsigned int cpu_usage;
    double memory_used;
    double memory_total;
    double network_in;
    double network_out;
    double disk_used;
    double disk_total;
} HardwareStats;

HardwareStats init_hardware_stats() {
    HardwareStats stats = {0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0};
    return stats;
}

int get_cpu_usage(unsigned int* result) {
    natural_t cpu_count;
    processor_cpu_load_info_t cpu_load1, cpu_load2;
    mach_msg_type_number_t info_count1, info_count2;

    // Primeira amostra
    kern_return_t err = host_processor_info(
        mach_host_self(), PROCESSOR_CPU_LOAD_INFO, &cpu_count,
        (processor_info_array_t*)&cpu_load1, &info_count1
    );
    if (err != KERN_SUCCESS) {
        printf("Erro na primeira amostra: %d\n", err);
        return -1;
    }

    sleep(1);

    err = host_processor_info(
        mach_host_self(), PROCESSOR_CPU_LOAD_INFO, &cpu_count,
        (processor_info_array_t*)&cpu_load2, &info_count2
    );
    if (err != KERN_SUCCESS) {
        printf("Erro na segunda amostra: %d\n", err);
        vm_deallocate(mach_task_self(), (vm_address_t)cpu_load1, info_count1);
        return -1;
    }

    unsigned int total_usage = 0;
    for (natural_t i = 0; i < cpu_count; i++) {
        unsigned int user_diff = cpu_load2[i].cpu_ticks[CPU_STATE_USER] - cpu_load1[i].cpu_ticks[CPU_STATE_USER];
        unsigned int system_diff = cpu_load2[i].cpu_ticks[CPU_STATE_SYSTEM] - cpu_load1[i].cpu_ticks[CPU_STATE_SYSTEM];
        unsigned int idle_diff = cpu_load2[i].cpu_ticks[CPU_STATE_IDLE] - cpu_load1[i].cpu_ticks[CPU_STATE_IDLE];
        unsigned int total_diff = user_diff + system_diff + idle_diff;
        if (total_diff > 0) {
            total_usage += (user_diff + system_diff) * 100 / total_diff;
        }
    }

    vm_deallocate(mach_task_self(), (vm_address_t)cpu_load1, info_count1);
    vm_deallocate(mach_task_self(), (vm_address_t)cpu_load2, info_count2);
    *result = total_usage / (cpu_count > 0 ? cpu_count : 1);
    return 0;
}

int get_memory_stats(double* used, double* total) {
    vm_statistics64_data_t vm_stat;
    mach_msg_type_number_t count = HOST_VM_INFO64_COUNT;

    kern_return_t err = host_statistics64(
        mach_host_self(), HOST_VM_INFO64, (host_info64_t)&vm_stat, &count
    );
    if (err != KERN_SUCCESS) {
        printf("Erro em get_memory_stats: %d\n", err);
        return -1;
    }

    double page_size = (double)vm_page_size;
    *used = (double)(
        vm_stat.active_count + 
        vm_stat.wire_count + 
        vm_stat.inactive_count + 
        vm_stat.compressor_page_count
    ) * page_size / 1073741824.0;
    *used = round(*used * 10.0) / 10.0;

    int mib[2] = {CTL_HW, HW_MEMSIZE};
    int64_t mem_size;
    size_t len = sizeof(mem_size);
    if (sysctl(mib, 2, &mem_size, &len, NULL, 0) != 0) {
        printf("Erro em get_memory_stats: sysctl falhou\n");
        return -1;
    }

    *total = (double)mem_size / 1073741824.0;
    *total = round(*total * 10.0) / 10.0;
    return 0;
}

int get_network_stats(double* in_mb, double* out_mb) {
    struct ifaddrs* ifaddr;
    if (getifaddrs(&ifaddr) != 0) {
        printf("Erro em get_network_stats: getifaddrs falhou\n");
        return -1;
    }

    unsigned long long in_bytes = 0;
    unsigned long long out_bytes = 0;
    struct ifaddrs* current = ifaddr;
    while (current != NULL) {
        if (current->ifa_data != NULL) {
            struct if_data* if_data = (struct if_data*)current->ifa_data;
            in_bytes += if_data->ifi_ibytes;
            out_bytes += if_data->ifi_obytes;
        }
        current = current->ifa_next;
    }
    freeifaddrs(ifaddr);

    *in_mb = (double)in_bytes / 1048576.0;
    *out_mb = (double)out_bytes / 1048576.0;

    *in_mb = round(*in_mb * 10.0) / 10.0;
    *out_mb = round(*out_mb * 10.0) / 10.0;
    return 0;
}

int get_disk_stats(Config config, double* used, double* total) {
    if (!config.settings.show_disk) {
        *used = 0.0;
        *total = 0.0;
        return 0;
    }

    DASessionRef session = DASessionCreate(kCFAllocatorDefault);
    if (session == NULL) {
        printf("Erro em get_disk_stats: DASessionCreate falhou\n");
        return -1;
    }

    DADiskRef disk = DADiskCreateFromBSDName(kCFAllocatorDefault, session, "/dev/disk0");
    if (disk == NULL) {
        printf("Erro em get_disk_stats: DADiskCreateFromBSDName falhou\n");
        CFRelease(session);
        return -1;
    }

    CFDictionaryRef disk_info = DADiskCopyDescription(disk);
    if (disk_info == NULL) {
        printf("Erro em get_disk_stats: DADiskCopyDescription retornou NULL\n");
        CFRelease(disk);
        CFRelease(session);
        return -1;
    }

    CFNumberRef size_ref = CFDictionaryGetValue(disk_info, kDADiskDescriptionMediaSizeKey);
    if (size_ref == NULL) {
        printf("Erro em get_disk_stats: kDADiskDescriptionMediaSizeKey não encontrado\n");
        CFRelease(disk_info);
        CFRelease(disk);
        CFRelease(session);
        return -1;
    }

    int64_t disk_size;
    CFNumberGetValue(size_ref, kCFNumberSInt64Type, &disk_size);

    CFRelease(disk_info);
    CFRelease(disk);
    CFRelease(session);

    *total = (double)disk_size / 1000000000.0;
    *total = round(*total * 10.0) / 10.0;

    struct statfs stat;
    if (statfs("/", &stat) != 0) {
        printf("Erro em get_disk_stats: statfs falhou\n");
        return -1;
    }

    double block_size = (double)stat.f_bsize;
    *used = (double)(stat.f_blocks - stat.f_bfree) * block_size / 1000000000.0;
    *used = round(*used * 10.0) / 10.0;

    return 0;
}

int get_hardware_stats(Config config, HardwareStats* stats) {
    *stats = init_hardware_stats();

    if (config.settings.show_cpu && get_cpu_usage(&stats->cpu_usage) != 0) {
        return -1;
    }

    double mem_used, mem_total;
    if (config.settings.show_memory && get_memory_stats(&mem_used, &mem_total) != 0) {
        return -1;
    }
    stats->memory_used = config.settings.show_memory ? mem_used : 0.0;
    stats->memory_total = config.settings.show_memory ? mem_total : 0.0;

    double net_in, net_out;
    if (get_network_stats(&net_in, &net_out) != 0) {
        return -1;
    }
    stats->network_in = net_in;
    stats->network_out = net_out;

    double disk_used, disk_total;
    if (get_disk_stats(config, &disk_used, &disk_total) != 0) {
        return -1;
    }
    stats->disk_used = disk_used;
    stats->disk_total = disk_total;

    return 0;
}