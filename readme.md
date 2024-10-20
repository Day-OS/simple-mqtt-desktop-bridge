## Publish:

### Notify Linux computer

**Topic:** `smqttdb-notify` for all devices or `smqttdb-notify-<DEVICE_ID>` for a specific device
```json
{
  "summary": "Title",
  "body": "The body of your notification"
}
```

**Home Assistant Example:**
```yml
    - action: mqtt.publish
    metadata: {}
    data:
      evaluate_payload: false
      qos: 0
      retain: false
      payload: |-
        {
            "summary": "Title",
            "body": "The body of your notification"
            }
      topic: notify
```

### Shutdown Computer
**Topic:** `smqttdb-sleep` for all devices or `smqttdb-sleep-<DEVICE_ID>` for a specific device
```json
{
  "time": "sleep command arguments. Ex: 'now', '+2', '8:00' "
}
```


## Subscribe:
### Computer Specifications

**Topic:** `smqttdb-specs`
```json
{
    "free_memory": 0,
    "free_swap": 0,
    "used_swap": 0,
    "total_swap": 0,
    "physical_core_count": 0,
    "total_memory": 0,
    "available_memory": 0,
    "used_memory": 0,
    "cpus": [
        {
            "name": "cpu0",
            "frequency": 0,
            "vendor_id": "Vendor name",
            "brand": "Your CPU"
        }
    ],
    "global_cpu_usage": 0
}

```
`configuration.yml`
```yml
mqtt:
  sensor:
    - name: "Computer Used Memory"
      state_topic: "smqttdb-specs"
      value_template: "{{ value_json.used_memory | float / 1000000 | round(0) }}"
      unit_of_measurement: "MB"
```



## Environment Variables

`IP=<MQTT Broker IP>`
`PORT=<MQTT broker PORT, usually 1883>`
`SPECS_INTERVAL=<Interval in Miliseconds in which subscriptions updates will be sent to broker>`