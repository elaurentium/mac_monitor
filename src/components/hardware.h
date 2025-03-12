#ifndef HARDWARE_H
#define HARDWARE_H

#include <stdbool.h>
#include <stdint.h>
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

HardwareStats init_hardware_stats();
int get_hardware_stats(Config config, HardwareStats* stats);

#endif // HARDWARE_H