#ifndef SYSTEM_MONITOR_H
#define SYSTEM_MONITOR_H

#include <stdint.h>

typedef struct {
    double cpu_usage;
    uint64_t cpu_freq;
    double cpu_temp;
    double gpu_temp;
    uint64_t ram_usage;
    uint64_t ram_total;
} SystemMetrics;

SystemMetrics run_system_monitor();

#endif // SYSTEM_MONITOR_H