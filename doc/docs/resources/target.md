# Target

Targets(List)

- The field is a list of target. Each target include name.

  

Target for specifying the package.

| Attribute | Value Type |
| --------- | ---------- |
| name      | string     |



### Attribute Description



#### *name

------

- ###### Value Type

  string

- ###### Mandatory option

  required

- ###### Description

  This field is useful for specifying the target, which indicates which package needs to be executed.

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
  targets:
    - name: "lges-cloud"
```