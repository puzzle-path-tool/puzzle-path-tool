pub mod ts_api;


pub mod layers {
    pub mod id_helpers;
    pub(crate) mod second_layer;
    pub mod third_layer;
}

/* 
#[cfg(test)]
#[allow(clippy::expect_used, clippy::unwrap_used, reason = "clippy-test-cfg")]
mod tests {
    use crate::layers::second_layer::steps::LogicStep as SecondLayerLogicStep;
    use crate::layers::second_layer::tables::TableBundle as SecondLayerTableBundle;
    use crate::layers::third_layer::steps::LogicStep as ThirdLayerLogicStep;
    use crate::layers::third_layer::tables::TableBundle as ThirdLayerTableBundle;
    #[test]
    fn it_works() {
        let second_layer_tables = build_second_layer_tables(3);
        //println!("{second_layer_tables:#?}");
        let second_layer_step = build_second_layer_step(&second_layer_tables);
        println!("{second_layer_step:#?}");
        let third_layer_tables = build_third_layer_tables(&second_layer_tables);
        //println!("{third_layer_tables:#?}");
        let third_layer_step = build_third_layer_step(&second_layer_step, &second_layer_tables);
        println!("{third_layer_step:#?}");
    }

    fn build_third_layer_step(
        second_layer_step: &SecondLayerLogicStep,
        second_layer_tables: &SecondLayerTableBundle,
    ) -> ThirdLayerLogicStep {
        ThirdLayerLogicStep::from_second_layer(second_layer_step, second_layer_tables)
    }

    fn build_third_layer_tables(
        second_layer_tables: &SecondLayerTableBundle,
    ) -> ThirdLayerTableBundle {
        ThirdLayerTableBundle::from_second_layer(second_layer_tables)
    }

    fn build_second_layer_step(
        second_layer_tables: &SecondLayerTableBundle,
    ) -> SecondLayerLogicStep {
        use crate::layers::second_layer::steps::*;
        test::build_example_step(second_layer_tables)
    }

    fn build_second_layer_tables(amount: u32) -> SecondLayerTableBundle {
        use crate::layers::second_layer::tables::*;

        let mut tables: Vec<(String, String, Vec<(String, TODO_FieldTypeStandIn)>)> = vec![];
        let mut enum_tables: Vec<(
            String,
            String,
            Vec<String>,
            (Vec<Vec<(String, i32)>>, Vec<Vec<(i32, String)>>),
        )> = vec![];

        for n in 1..=amount {
            enum_tables.push((
                format!("EnumName{n}"),
                format!("EnumDescription{n}"),
                vec![
                    format!("EnumValue{n}.1",),
                    format!("EnumValue{n}.2",),
                    format!("EnumValue{n}.3",),
                ],
                (vec![], vec![]),
            ));

            tables.push((
                format!("TableName{n}"),
                format!("TableDescription{n}"),
                vec![
                    (
                        format!("FieldName{n}.1"),
                        TODO_FieldTypeStandIn::Array(TODO_FlatFieldTypeStandIn::Primitive(
                            TODO_PrimitiveFieldType::Enum("EnumName1".to_string()),
                        )),
                    ),
                    (
                        format!("FieldName{n}.2"),
                        TODO_FieldTypeStandIn::Flat(TODO_FlatFieldTypeStandIn::Object(vec![
                            (
                                format!("ObjectFieldName{n}.2.1"),
                                TODO_FieldTypeStandIn::Flat(TODO_FlatFieldTypeStandIn::Primitive(
                                    TODO_PrimitiveFieldType::Number,
                                )),
                            ),
                            (
                                format!("ObjectFieldName{n}.2.2"),
                                TODO_FieldTypeStandIn::Flat(TODO_FlatFieldTypeStandIn::Primitive(
                                    TODO_PrimitiveFieldType::Boolean,
                                )),
                            ),
                        ])),
                    ),
                    (
                        format!("FieldName{n}.3"),
                        TODO_FieldTypeStandIn::Flat(TODO_FlatFieldTypeStandIn::Primitive(
                            TODO_PrimitiveFieldType::Enum(format!("EnumName{n}")),
                        )),
                    ),
                ],
            ));
        }

        TableBundle::new(&tables, enum_tables)
    }
}*/

