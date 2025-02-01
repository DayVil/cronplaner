# Cronplaner

A simple calendar planer for the terminal. It uses a config file to configure appointments.

## How to Use

Create `~/.config/cronplaner/appointments.toml` or set another path by setting `CRONPLANER_CONFIG_DIR` var to a custom path. However the name
of the config cannot be changed.
Each entry is set by providing:

```toml
[[time_slots]]
name = "Any Name"
date = "2025-02-10"
time = "10:00:00" # Optional
time_zone = "Europe/Berlin" # Optional
```


