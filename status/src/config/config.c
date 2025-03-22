#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

typedef struct {
    unsigned long update_interval_secs;
    bool show_cpu;
    bool show_memory;
    bool show_disk;
} Settings;

typedef struct {
    Settings settings;
} Config;

Config* config_load(const char* path) {
    FILE* file = fopen(path, "r");
    if (!file) {
        perror("Failed to open config file");
        return NULL;
    }

    Config* config = (Config*)malloc(sizeof(Config));
    if (!config) {
        fclose(file);
        return NULL;
    }


    config->settings.update_interval_secs = 2;
    config->settings.show_cpu = true;
    config->settings.show_memory = true;
    config->settings.show_disk = true;


    char line[256];
    while (fgets(line, sizeof(line), file)) {
        line[strcspn(line, "\n")] = 0;

        if (strstr(line, "update_interval_secs")) {
            sscanf(line, "update_interval_secs = %lu", &config->settings.update_interval_secs);
        } else if (strstr(line, "show_cpu")) {
            config->settings.show_cpu = (strstr(line, "true") != NULL);
        } else if (strstr(line, "show_memory")) {
            config->settings.show_memory = (strstr(line, "true") != NULL);
        } else if (strstr(line, "show_disk")) {
            config->settings.show_disk = (strstr(line, "true") != NULL);
        }
    }

    fclose(file);
    return config;
}

Config config_default() {
    Config config;
    config.settings.update_interval_secs = 2;
    config.settings.show_cpu = true;
    config.settings.show_memory = true;
    config.settings.show_disk = true;
    return config;
}

void config_free(Config* config) {
    if (config) {
        free(config);
    }
}