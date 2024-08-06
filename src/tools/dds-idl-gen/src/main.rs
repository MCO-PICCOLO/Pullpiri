fn main() {
    println!("Hello, world!\n");
    let idl_src = r#"module dayTime {
    @final
    struct DataType {
        bool day;
    };
};"#;
    let result = dust_dds_gen::compile_idl(idl_src).unwrap();
    println!("{}\n", result);

    let idl_src = r#"module gearState {
    @final
    struct DataType {
        string state;
    };
};"#;
    let result = dust_dds_gen::compile_idl(idl_src).unwrap();
    println!("{}\n", result);

    let idl_src = r#"module lightState {
    @final
    struct DataType {
        bool on;
    };
};"#;
    let result = dust_dds_gen::compile_idl(idl_src).unwrap();
    println!("{}", result);

    let idl_src = r#"module speed {
    @final
    struct DataType {
        int value;
    };
};"#;
    let result = dust_dds_gen::compile_idl(idl_src).unwrap();
    println!("{}", result);

    let idl_src = r#"module turnLight {
    @final
    struct DataType {
        string operation;
    };
};"#;
    let result = dust_dds_gen::compile_idl(idl_src).unwrap();
    println!("{}", result);
}
/*
# TOPIC NAME = /rt/piccolo/Day_Time
pub mod dayTime {
    #[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
    pub struct DataType {
        pub day: bool,
    }
}

# TOPIC NAME = /rt/piccolo/Gear_State
pub mod gearState {
    #[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
    pub struct DataType {
        pub state: String,
    }
}

# TOPIC NAME = /rt/piccolo/Light_State
pub mod lightState {
    #[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
    pub struct DataType {
        pub on: bool,
    }
}

# TOPIC NAME = /rt/piccolo/Speed
pub mod speed {
    #[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
    pub struct DataType {
        pub value: int,
    }
}

# TOPIC NAME = /rt/piccolo/Turn_Light
pub mod turnLight {
    #[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
    pub struct DataType {
        pub operation: String,
    }
}
*/