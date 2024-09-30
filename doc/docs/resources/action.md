# Action



Action for operating scenario.

| Attribute | Value Type |
| --------- | ---------- |
| operation | string     |





### Attribute Description



#### *operation

------

- ###### Value Type

  string

- ###### Mandatory option

  required

- ###### Description

  This field is useful for specifying the actions that need to be performed when the conditions are met. The types of operations include `upload-cloud`, `move-node`, and others. It is essential to specify the actions for the scenario.

**scenario_cloud_upload.yaml**

```
apiVersion: v1
kind: Scenario
metadata:
  name: upload-battery-state-cloud
spec:
  conditions:
    andcombinations:
      - express: Equal
        value: "off"
        operands:
          type: DDS
          name: acc_state
          value: "rt/piccolo/acc_state"
  action:
    - operation: upload-cloud
```