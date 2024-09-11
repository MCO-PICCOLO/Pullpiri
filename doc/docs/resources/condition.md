Scenario Spec

# Conditions

- Conditions(List)

  - The field is a list of condition. Each condition includes `express`, `value`, and `operands`.

    

| Attribute | Value Type |
| --------- | ---------- |
| express   | string     |
| value     | string     |
| operand   | list       |



### Attribute Description

#### *express

------

- ###### Value Type

  string

- ###### Mandatory option

  required

- ###### Description

  The `express` field specifies how to compare the value.

  - ###### Equal (string)

    The `Equal` expression is useful for checking if the state value is equal. 

  - ###### GT (string)

     The `gt` expression is useful for comparing if a specific value, such as speed, is greater than a certain threshold.

  - ###### LT (string)

     The `st` expression is useful for comparing if a specific value is less than or equal to a certain threshold.



#### *value

------

- ###### Value Type

  string

- ###### Mandatory option

  required

- ###### Description

  The `value` field specifies the data of a specific module. For example, it can contain a numeric value for speed or an `on`/`off` state for acceleration (acc). 

  

#### *operands

------

- ###### Value Type

  list

- ###### Mandatory option

  required

- ###### Description

  This field is useful when writing complex scenarios using `and` and `or` combinations. The default is `andCombination`.

  - ###### operands.type (string)

    The `type` field within `operands` is useful for specifying the communication method to be used. such as `DDS` and others

  - ###### operands.name (string)

     The `name` field specifies a particular module of the vehicle.

  - ###### operands.value (string)

     The `value` field specifies the topic for dds with the vehicle module.

    

**scenario_cloud_upload.yaml**

```
apiVersion: v1
kind: Scenario
metadata:
  name: move-zonal-lges-cloud
spec:
  conditions:
    express: Equal
    value: "off"
    operands:
      type: DDS
      name: acc_state
      value: "rt/piccolo/acc_state"
```

