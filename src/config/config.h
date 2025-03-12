#ifndef CONFIG_H
#define CONFIG_H

#include <stdbool.h>
#include <stdint.h>

typedef struct {
    uint64_t update_interval_secs;
    bool show_cpu;
    bool show_memory;
    bool show_disk;
} Settings;

typedef struct {
    Settings settings;
} Config;

Config load_config(const char *path);
Config default_config();

#endif // CONFIG_H