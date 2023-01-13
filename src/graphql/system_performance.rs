use juniper::{GraphQLObject};
use sysinfo::{ComponentExt, CpuExt, NetworkExt, ProcessExt, System, SystemExt};

#[derive(GraphQLObject)]
#[graphql(description = "Memort Information")]
pub struct MemoryInfo {
    total_memory: i32,
    used_memory: i32,
    available_memory: i32,
    total_swap: i32,
    used_swap: i32,
}

#[derive(GraphQLObject)]
#[graphql(description = "CPU Information")]
pub struct CPUUsage {
    name: String,
    usage: f64,
}


#[derive(GraphQLObject)]
#[graphql(description = "CPU Information")]
pub struct CPUInfo {
    total_core: i32,
    cpu: Vec<CPUUsage>,
}

#[derive(GraphQLObject)]
#[graphql(description = "Process Information")]
pub struct ProcessInfo {
    name: String,
    pid: String,
    cpu_usage: f64,
    memory: f64,
    virtual_memory: f64,
}


#[derive(GraphQLObject)]
#[graphql(description = "Network Info Object that object stores the system information")]
pub struct NetworkInfo {
    name: String,
    received: f64,
    sent: f64,
}

#[derive(GraphQLObject)]
#[graphql(description = "Component Info Object that object stores the system information")]
pub struct ComponentInfo {
    name: String,
    temperature: f64,
}

#[derive(GraphQLObject)]
#[graphql(description = "System Data Object that object stores the system information")]
pub struct SystemData {
    name: String,
    kernel_version: String,
    os_version: String,
    host_name: String,
}


#[derive(GraphQLObject)]
#[graphql(description = "System Info Object that object stores the system information")]
pub struct SystemInfo {
    memory: MemoryInfo,
    cpu: CPUInfo,
    process: Vec<ProcessInfo>,
    network: Vec<NetworkInfo>,
    component: Vec<ComponentInfo>,
    system: SystemData,
}

pub fn get_system_info() -> SystemInfo {
    let sys = System::new_all();


    let mut cpudata: Vec<CPUUsage> = Vec::new();
    for cpu in sys.cpus() {
        cpudata.push(CPUUsage {
            name: cpu.name().to_string(),
            usage: cpu.cpu_usage() as f64,
        });
    }

    let mut process_data: Vec<ProcessInfo> = Vec::new();
    for (pid, process) in sys.processes() {
        process_data.push(ProcessInfo {
            name: process.name().to_string(),
            pid: pid.to_string(),
            cpu_usage: process.cpu_usage() as f64,
            memory: process.memory() as f64,
            virtual_memory: process.virtual_memory() as f64,
        });
    }

    let mut network_data: Vec<NetworkInfo> = Vec::new();
    for (interface_name, data) in sys.networks() {
        network_data.push(NetworkInfo {
            name: interface_name.to_string(),
            received: data.received() as f64,
            sent: data.transmitted() as f64,
        });
    }

    let mut component_data: Vec<ComponentInfo> = Vec::new();
    for component in sys.components() {
        component_data.push(ComponentInfo {
            name: component.label().to_string(),
            temperature: component.temperature() as f64,
        });
    }

    SystemInfo {
        memory: MemoryInfo {
            total_memory: sys.total_memory() as i32,
            used_memory: sys.used_memory() as i32,
            available_memory: sys.available_memory() as i32,
            total_swap: sys.total_swap() as i32,
            used_swap: sys.used_swap() as i32,
        },
        cpu: CPUInfo {
            total_core: sys.cpus().len() as i32,
            cpu: cpudata,
        },
        process: process_data,
        network: network_data,
        component: component_data,
        system: SystemData {
            name: sys.name().unwrap(),
            kernel_version: sys.kernel_version().unwrap(),
            os_version: sys.os_version().unwrap(),
            host_name: sys.host_name().unwrap(),
        },
    }
}
