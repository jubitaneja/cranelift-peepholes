fn peep_opt (pos: &mut FuncCursor, inst: Inst) {
    match pos.func.dfg[inst] {
        InstructionData::Binary { opcode, args } => {
            match opcode {
                Opcode::Iadd => {
                    match pos.func.dfg.value_def(args[0]) {
                        ValueDef::Param(_, _) => {
                            match pos.func.dfg.value_def(args[1]) {
                                ValueDef::Result(arg_ty, _) => {
                                    match pos.func.dfg[arg_ty] {
                                        InstructionData::Binary {opcode, args} => {
                                            match opcode {
                                                Opcode::Iadd => {
                                                    match pos.func.dfg.value_def(args[0]) {
                                                        ValueDef::Param(_, _) => {
                                                            match pos.func.dfg.value_def(args[1]) {
                                                                ValueDef::Param(_, _) => {
                                                                    pos.func.dfg
                                                                            .replace(inst)
                                                                            .imul_imm(args[1], 3);
                                                                },
                                                                _ => {}
                                                            }
                                                        },
                                                        _ => {}
                                                    }
                                                },
                                                _ => {}
                                            }
                                        },
                                        _ => {}
                                    }
                                },
                                _ => {}
                            }
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        },
        _ => {}
    }
}
