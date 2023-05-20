# RBR2G29

This is a small tool that maps the Richard Burns Rally (NGP) RPM data to the Logitech G27 and G29 LED bars. It's heavily based on Andris0's [DR2G27](https://github.com/Andris0/DR2G27).

## Requirements

Activate UDP telemetry in the RSF launcher and set IP and port to: `127.0.01:6776`

## Telemetry

By default `127.0.01:6776` is used to listen for telemetry. Using the `-i --ip` and `-p --port` parameters, a custom adress can be defined. For example:

```
.\rbr2g29.exe -i 192.168.0.20 -p 1234
```

## Download

A binary is available in the repository here: https://github.com/lennartb-/RBR2G29/blob/main/bin/rbr2g29.exe
