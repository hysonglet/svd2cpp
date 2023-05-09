use std::env::args;
// use std::fmt::format;
use std::fs::{self, File};
// use std::io::prelude::*;
use std::io::{Read, Write};
use svd::svd::{ClusterInfo, RegisterInfo};
use svd::svd::{
    Device, FieldInfo, MaybeArray,
    MaybeArray::{Array, Single},
    Name, PeripheralInfo, RegisterCluster,
};
use svd_parser as svd;

struct Cpp {
    content: String,
    tab: isize,
    fs_device: File, // 头文件
    fs_periph: File, //外设文件
}

#[allow(dead_code)]
#[derive(PartialEq, Eq)]
enum Tab {
    INC,
    DEC,
    KEEP,
}

impl Cpp {
    pub fn new(str: &str) -> Cpp {
        let mut c = Cpp {
            content: String::new(),
            tab: 0,
            fs_periph: fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open("ReadMe.md")
                .unwrap(),
            fs_device: fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(str)
                .unwrap(),
        };

        c.fs_periph.write("## Hysonglet".as_bytes()).unwrap();

        c
    }
    pub fn append(&mut self, str: &str, pre_tab: Tab, tab: Tab) {
        let mut s = String::new();
        match pre_tab {
            Tab::DEC => {
                self.tab -= 1;
                if self.tab < 0 {
                    self.tab = 0;
                }
            }
            Tab::INC => self.tab += 1,
            Tab::KEEP => (),
        }

        for _ in 0..self.tab {
            // s.push('    ');
            s.push_str("    ");
        }
        s.push_str(str);
        s.push('\n');

        self.content.push_str(s.as_str());
        self.fs_periph.write(s.as_bytes()).unwrap();
        match tab {
            Tab::DEC => {
                self.tab -= 1;
                if self.tab < 0 {
                    self.tab = 0;
                }
            }
            Tab::INC => self.tab += 1,
            Tab::KEEP => (),
        }
    }

    pub fn append_some_coment(&mut self, des: &Option<String>) {
        if let Some(str) = des {
            let mut s = String::new();
            for _ in 0..self.tab {
                s.push('\t');
            }
            s.push_str("/* ");
            s.push_str(str);
            s.push_str(" */");
            s.push('\n');

            self.content.push_str(s.as_str());
            self.fs_periph.write(s.as_bytes()).unwrap();
        }
    }

    fn append_coment(&mut self, des: &str) {
        let mut s = String::new();
        for _ in 0..self.tab {
            // s.push('\t');
            s.push_str("    ");
        }
        s.push_str("/* ");
        s.push_str(des);
        s.push_str(" */");
        s.push('\n');

        self.content.push_str(s.as_str());
        self.fs_periph.write(s.as_bytes()).unwrap();
    }

    fn device_file_append(&mut self, str: &str) {
        self.fs_device.write(str.as_bytes()).unwrap();
        self.fs_device.write("\n".as_bytes()).unwrap();
    }

    fn device_file_append_header(&mut self, dev_name: &str) {
        self.device_file_append(format!("#ifndef _PAC_{}_H", dev_name.to_uppercase()).as_str());
        self.device_file_append(format!("#define _PAC_{}_H", dev_name.to_uppercase()).as_str());
        self.device_file_append(format!("#include <stdint.h>").as_str());
    }

    fn peripheral_file_append_header(&mut self, peri: &str) {
        let peri_macro = format!("_PAC_{}_H", peri.to_uppercase());

        self.append(
            format!("#ifndef {peri_macro}\n#define {peri_macro}").as_str(),
            Tab::KEEP,
            Tab::KEEP,
        );
        self.append(
            format!("#include \"register.h\"\n#include <stdint.h>").as_str(),
            Tab::KEEP,
            Tab::KEEP,
        );
    }

    fn fs_device_close(&mut self) {
        self.fs_device.write(format!("#endif").as_bytes()).unwrap();
        self.fs_device.sync_all().unwrap();
    }

    fn fs_periph_create(&mut self, str: &str) {
        self.fs_periph = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(str)
            .unwrap();
    }

    fn fs_periph_close(&mut self) {
        self.fs_periph.write(format!("#endif").as_bytes()).unwrap();
        self.fs_periph.sync_all().unwrap();
    }

    #[allow(dead_code)]
    fn display(&self) {
        println!("{}", self.content);
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    let file_name = args.get(1).unwrap();

    let xml = &mut String::new();
    _ = File::open(file_name).unwrap().read_to_string(xml);

    let device = svd::parse(xml).unwrap();

    let _ = fs::create_dir(format!("{}", device.name().to_lowercase()));
    // 创建设备总头文件
    let mut cpp = Cpp::new(
        format!(
            "{}/{}_pac.h",
            device.name().to_lowercase(),
            device.name().to_lowercase()
        )
        .as_str(),
    );

    cpp.device_file_append_header(device.name());

    for peripheral in &device.peripherals {
        println!("pack [{}]", peripheral.name());

        match peripheral {
            Single(peripheral) => {
                periph_header_file_create(&device, &peripheral, &mut cpp);
                device_header_file_add_peri(&peripheral, &mut cpp);
                if periph_derived(&peripheral, &mut cpp) {
                    continue;
                }
                periph_struct_create(&peripheral, &mut cpp);
            }
            Array(_, _) => {}
        }

        cpp.fs_periph_close();
    }
    cpp.fs_device_close();
}

fn periph_header_file_create(dev: &Device, peri: &PeripheralInfo, cpp: &mut Cpp) {
    cpp.fs_periph_create(
        format!(
            "{}/pac_{}.h",
            dev.name().to_lowercase(),
            peri.name().to_lowercase()
        )
        .as_str(),
    );
    // 添加外设头文件的文件头
    cpp.peripheral_file_append_header(peri.name());

    /* 外设描述注释 */
    if let Some(des) = &peri.description {
        cpp.append(format!("/* {} */", des).as_str(), Tab::KEEP, Tab::KEEP);
    }
}

fn device_header_file_add_peri(peripheral: &PeripheralInfo, cpp: &mut Cpp) {
    let peripheral_name = peripheral.name();
    //设备头文件添加外设头文件
    cpp.device_file_append(
        format!("#include \"pac_{}.h\"", peripheral_name.to_lowercase()).as_str(),
    );
    // 添加外设地址
    cpp.device_file_append(
        format!(
            "volatile {} *PAC_{} = ({} *)0x{:x};",
            peripheral_name.to_uppercase(),
            peripheral_name.to_uppercase(),
            peripheral_name.to_uppercase(),
            peripheral.base_address
        )
        .as_str(),
    );
}

fn periph_derived(peripheral: &PeripheralInfo, cpp: &mut Cpp) -> bool {
    if let Some(derived) = &peripheral.derived_from {
        let peripheral_name = peripheral.name();
        cpp.append(
            format!("#include \"pac_{}.h\"", derived.to_lowercase()).as_str(),
            Tab::KEEP,
            Tab::KEEP,
        );
        cpp.device_file_append(format!("/* {peripheral_name} derived from {derived} */").as_str());
        cpp.append(
            format!(
                "typedef {} {};",
                derived.to_uppercase(),
                peripheral_name.to_uppercase()
            )
            .as_str(),
            Tab::KEEP,
            Tab::KEEP,
        );
        cpp.fs_periph_close();
        // 不再继续解析外设寄存器，直接使用继承的就行
        true
    } else {
        false
    }
}

fn periph_struct_create(peripheral: &PeripheralInfo, cpp: &mut Cpp) {
    let peripheral_name = peripheral.name();
    cpp.append(
        format!("struct {} {{", peripheral_name).as_str(),
        Tab::KEEP,
        Tab::INC,
    );

    let mut reg_addr = 0;
    let mut rsv_reg = 0;
    let registers = &peripheral.registers;
    if let Some(registers) = registers {
        for registercluster in registers {
            // println!("{:x} {:x}", registercluster.address_offset(), reg_addr);
            if registercluster.address_offset() < reg_addr {
                // 寄存器地址相同，则忽略后一个寄存器
                continue;
            }
            let reg_cnt = registercluster.address_offset() - reg_addr;
            if reg_cnt != 0 {
                cpp.append(
                    format!("private: uint8_t rsv_{rsv_reg}[{}];", reg_cnt).as_str(),
                    Tab::KEEP,
                    Tab::KEEP,
                );
                rsv_reg += 1;
            }
            match registercluster {
                RegisterCluster::Register(register) => {
                    // println!("{}", register.name());
                    let reg_size = if let Some(size) = register.properties.size {
                        size / 8
                    } else {
                        4
                    };
                    match register {
                        // 寄存器类型为数组类型
                        MaybeArray::Array(register, dim) => {
                            let mut register = register.clone();
                            let len = register.name.len() - 5;
                            let struct_name = &register.name().clone()[0..len].to_string();
                            register.name = format!("{}[{}]", struct_name, dim.dim);
                            register_struct_create(struct_name, &register, cpp);
                            reg_addr = register.address_offset + reg_size * dim.dim;
                        }
                        // 寄存器为单个寄存器类型
                        MaybeArray::Single(register) => {
                            register_struct_create(register.name(), &register, cpp);
                            // println!(
                            //     "register.address_offset: {:x} reg_size: {}",
                            //     register.address_offset, reg_size
                            // );
                            reg_addr = register.address_offset + reg_size;
                        }
                    }
                }
                // 寄存器组
                RegisterCluster::Cluster(cluster) => {
                    // println!("{} is cluster", cluster.name());
                    let reg_all = cluster_register_strcut_create(cluster, cpp);
                    reg_addr = cluster.address_offset + reg_all;
                }
            }
        }
    }

    cpp.append(
        format!("}}; /* end of {} */ ", peripheral.name()).as_str(),
        Tab::DEC,
        Tab::KEEP,
    );
}

fn register_struct_create(strcut_name: &str, register: &RegisterInfo, cpp: &mut Cpp) {
    // 寄存器宽度，字节
    let reg_size = if let Some(size) = register.properties.size {
        size / 8
    } else {
        4
    };
    let reg_type = match reg_size {
        1 => "uint8_t",
        2 => "uint16_t",
        4 => "uint32_t",
        _ => panic!("error size: {}", reg_size),
    };
    // union REG_NAME {
    cpp.append_some_coment(&register.description);
    cpp.append(
        format!(
            "public: union {}{{ /* addr: 0x{:x} */ ",
            strcut_name.to_uppercase(),
            register.address_offset
        )
        .as_str(),
        Tab::KEEP,
        Tab::INC,
    );
    let properties = register.properties;
    // 读权限，添加读接口
    if let Some(acc) = properties.access {
        if acc == svd::svd::Access::ReadOnly {
            cpp.append(
                format!("ReadOnly<{reg_type}> r;").as_str(),
                Tab::KEEP,
                Tab::KEEP,
            );
        }
    }
    // 写权限，添加写接口
    if let Some(acc) = properties.access {
        if acc == svd::svd::Access::WriteOnce || acc == svd::svd::Access::WriteOnly {
            cpp.append(
                format!("WriteOnly<{reg_type}> r;").as_str(),
                Tab::KEEP,
                Tab::KEEP,
            );
        }
    }

    if let Some(acc) = properties.access {
        if acc == svd::svd::Access::ReadWrite {
            cpp.append(
                format!("ReadWrite<{reg_type}> r;").as_str(),
                Tab::KEEP,
                Tab::KEEP,
            );
        }
    }

    // 提前添加位域的枚举
    if let Some(fields) = &register.fields {
        register_fields_mask_enum_create(fields, cpp);
        register_fields_enum_create(fields, cpp);
        let reg_size = if let Some(size) = register.properties.size {
            size / 8
        } else {
            4
        };
        register_fields_struct_create(register, cpp, reg_size);
    }

    // for union
    cpp.append(
        format!("}}{};", register.name.to_uppercase()).as_str(),
        Tab::DEC,
        Tab::KEEP,
    );
}

fn register_fields_mask_enum_create(fields: &Vec<MaybeArray<FieldInfo>>, cpp: &mut Cpp) {
    cpp.append(
        format!("public: enum mask_t{{").as_str(),
        Tab::KEEP,
        Tab::INC,
    );
    for field in fields {
        // println!("{:?}", field.name());
        cpp.append(
            format!(
                "MASK_{:10} = 0x{:08x},",
                field.name(),
                ((0x1u64 << field.bit_width()) - 1) << field.bit_offset()
            )
            .as_str(),
            Tab::KEEP,
            Tab::KEEP,
        );
    }
    cpp.append(format!("}};").as_str(), Tab::DEC, Tab::KEEP);
}

fn register_fields_enum_create(fields: &Vec<MaybeArray<FieldInfo>>, cpp: &mut Cpp) {
    for field in fields {
        for vv in &field.enumerated_values {
            let ev = &vv.values;
            if ev.len() != 0 {
                cpp.append_some_coment(&field.description);
                cpp.append(
                    format!("public: enum {}_t {{", field.name().to_lowercase()).as_str(),
                    Tab::KEEP,
                    Tab::INC,
                );
                for ee in ev {
                    cpp.append_some_coment(&ee.description);
                    cpp.append(
                        format!(
                            "{}_{:10} = {},",
                            field.name.to_uppercase(),
                            ee.name().to_uppercase(),
                            ee.value.unwrap()
                        )
                        .as_str(),
                        Tab::KEEP,
                        Tab::KEEP,
                    );
                }
                cpp.append(format!("}};").as_str(), Tab::DEC, Tab::KEEP);
            }
        }
    }
}

fn register_fields_struct_create(register: &RegisterInfo, cpp: &mut Cpp, reg_size: u32) {
    // let reg_size = register.properties.size.unwrap() / 8;
    let fields = register.fields();
    let reg_type = match reg_size {
        1 => "uint8_t",
        2 => "uint16_t",
        4 => "uint32_t",
        _ => panic!("error size: {}", reg_size),
    };
    cpp.append(
        format!("public: struct fields{{").as_str(),
        Tab::KEEP,
        Tab::INC,
    );
    let mut offset_idx = 0;
    // println!("{}", fields.
    let mut v: Vec<&MaybeArray<FieldInfo>> = Vec::new();
    for field in fields{
        v.push(field);
    }
    v.sort_by(|a, b| a.bit_offset().cmp(&b.bit_offset()));
    let fields = v;
    for field in fields {
        if offset_idx != field.bit_offset() {
            // 添加保留字段
            cpp.append(
                format!(
                    "private: {:10} : {};",
                    reg_type,
                    field.bit_offset() - offset_idx
                )
                .as_str(),
                Tab::KEEP,
                Tab::KEEP,
            );
        }
        offset_idx = field.bit_offset() + field.bit_width();
        cpp.append_some_coment(&field.description);
        // 添加位域
        cpp.append(
            format!(
                "private: {:10} _{} : {};",
                reg_type,
                field.name().to_lowercase(),
                field.bit_width()
            )
            .as_str(),
            Tab::KEEP,
            Tab::KEEP,
        );

        let mut rw_t = String::from(reg_type);
        let enums = &field.enumerated_values;
        for vv in enums {
            let ev = &vv.values;
            if ev.len() != 0 {
                rw_t = format!("{}_t", field.name.to_lowercase());
            }
        }

        // 域，读权限，添加读接口
        if let Some(acc) = field.access {
            if acc == svd::svd::Access::ReadOnly
                || acc == svd::svd::Access::ReadWrite
                || acc == svd::svd::Access::ReadWriteOnce
            {
                if let Some(d) = &field.description {
                    cpp.append_coment(
                        format!(
                            "read fields {}.{}:{rw_t}({} bit)\t({d})",
                            register.name(),
                            field.name(),
                            field.bit_width()
                        )
                        .as_str(),
                    );
                }
                cpp.append(
                    format!(
                        "public: inline {rw_t} r_{}(void){{return ({rw_t})_{};}}",
                        field.name().to_lowercase(),
                        field.name().to_lowercase()
                    )
                    .as_str(),
                    Tab::KEEP,
                    Tab::KEEP,
                );
            }
        }
        // 写权限，添加写接口
        if let Some(acc) = field.access {
            if acc == svd::svd::Access::WriteOnce
                || acc == svd::svd::Access::WriteOnly
                || acc == svd::svd::Access::ReadWrite
            {
                if let Some(d) = &field.description {
                    cpp.append_coment(
                        format!(
                            "write fields {}.{}:{rw_t}({} bit)\t({d})",
                            register.name(),
                            field.name(),
                            field.bit_width()
                        )
                        .as_str(),
                    );
                }
                cpp.append(
                    format!(
                        "public: inline void w_{}({rw_t} var){{_{} = var & 0x{:x};}}",
                        field.name().to_lowercase(),
                        field.name().to_lowercase(),
                        ((1u128 << field.bit_width()) - 1)
                    )
                    .as_str(),
                    Tab::KEEP,
                    Tab::KEEP,
                );
            }
        }
    }
    let reg_bit_width = reg_size * 8;
    if offset_idx != reg_bit_width {
        cpp.append(
            format!("private: {reg_type} : {};", reg_bit_width - offset_idx).as_str(),
            Tab::KEEP,
            Tab::KEEP,
        );
    }
    cpp.append("}fields;", Tab::DEC, Tab::KEEP);
}

fn cluster_register_strcut_create(cluster: &MaybeArray<ClusterInfo>, cpp: &mut Cpp) -> u32 {
    let mut reg_all = 0;
    cpp.append(
        format!(
            "struct {} {{ /* 0x{:x} */",
            cluster.name().to_uppercase(),
            cluster.address_offset
        )
        .as_str(),
        Tab::KEEP,
        Tab::INC,
    );

    let mut reg_addr = 0;
    let mut rsv_reg = 0;
    let registers = cluster.registers();
    registers.for_each(|registercluster| {
        // println!("{:x} {:x}", registercluster.address_offset, reg_addr);
        if registercluster.address_offset < reg_addr {
            // 寄存器地址相同，则忽略后一个寄存器
            return;
        }
        let reg_cnt = registercluster.address_offset - reg_addr;
        if reg_cnt != 0 {
            cpp.append(
                format!("private: uint8_t rsv_{rsv_reg}[{}];", reg_cnt).as_str(),
                Tab::KEEP,
                Tab::KEEP,
            );
            rsv_reg += 1;
        }
        // println!("{}", register.name());
        let reg_size = if let Some(size) = registercluster.properties.size {
            size / 8
        } else {
            4
        };
        match registercluster {
            // 寄存器类型为数组类型
            MaybeArray::Array(register, dim) => {
                let mut register = register.clone();
                let len = register.name.len() - 5;
                let struct_name = &register.name().clone()[0..len].to_string();
                register.name = format!("{}[{}]", struct_name, dim.dim);
                register_struct_create(struct_name, &register, cpp);
                reg_addr = register.address_offset + reg_size * dim.dim;
                reg_all += reg_size * dim.dim;
            }
            // 寄存器为单个寄存器类型
            MaybeArray::Single(register) => {
                register_struct_create(register.name(), &register, cpp);
                reg_addr = register.address_offset + reg_size;
                reg_all += reg_size;
            }
        }
    });

    cpp.append(format!("}};").as_str(), Tab::DEC, Tab::KEEP);
    reg_all
}
