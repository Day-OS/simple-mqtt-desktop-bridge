## Publish:

### Notify Linux computer

**Topic:** `smqttdb-notify`
```json
{
  "summary": "Title",
  "body": "The body of your notification"
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


## Environment Variables

`IP=<MQTT Broker IP>`
`PORT=<MQTT broker PORT, usually 1883>`
`SPECS_INTERVAL=<Interval in Miliseconds in which subscriptions updates will be sent to broker>`