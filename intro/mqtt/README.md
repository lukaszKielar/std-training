# MQTT

## Connecting to MQTT Server

MQTT Server is running on Raspberry Pi. In order to see what's happening I need to install mosquitto locally

```bash
brew install mosquitto
```

and then

```bash
mosquitto_sub -u user -P pass -h host -p 1883 -t "#" -v
```
