use config_client::protos::github::com::michaelhenkel::config_controller::pkg::apis::v1::config_controller_client::ConfigControllerClient;
use config_client::protos::github::com::michaelhenkel::config_controller::pkg::apis::v1::SubscriptionRequest;
use config_client::protos::github::com::michaelhenkel::config_controller::pkg::apis::v1;
use config_client::protos::ssd_git::juniper::net::contrail::cn2::contrail::pkg::apis::core::v1alpha1;
use tonic::transport::Channel;
use std::error::Error;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ConfigControllerClient::connect("http://127.0.0.1:20443").await.unwrap();
    consume_response(&mut client).await?;

    println!("Connected...now sleeping for 2 seconds...");
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    drop(client);

    println!("Disconnected...");
    Ok(())
}

fn get_node() -> String {
    if env::args().len() > 0 {
        let args: Vec<String> = env::args().collect();
        args[1].to_string()
    } else {
        "5b3s30".to_string()
    }
}

async fn consume_response(client: &mut ConfigControllerClient<Channel>) -> Result<(), Box<dyn Error>> {
    let request = tonic::Request::new(SubscriptionRequest {
        name: get_node(),
    });
    let mut stream = client
        .subscribe_list_watch(request)
        .await?
        .into_inner();
    while let Some(response) = stream.message().await? {
        let action = response.action();
        let res = get_resource(response);
        match action {
            v1::response::Action::Add => res.add(),
            v1::response::Action::Update => res.update(),
            v1::response::Action::Delete => res.delete(),
        }        
    }
    drop(stream);
    Ok(())
}

fn get_resource(response: v1::Response) -> Box<dyn ProcessResource>{
    match response.new.unwrap().resource.unwrap() {
        v1::resource::Resource::VirtualNetwork(res) => Box::new(res),
        v1::resource::Resource::VirtualMachineInterface(res) =>  Box::new(res),
        v1::resource::Resource::VirtualRouter(res) =>  Box::new(res),
    }

}


trait ProcessResource {
    fn kind(&self) -> String;
    fn add(&self);
    fn update(&self);
    fn delete(&self);
}
 
impl ProcessResource for v1alpha1::VirtualNetwork {
    fn kind(&self) -> String { "VirtualNetwork".to_string() }
    fn add(&self) { 
        println!("add for {} {}/{}",self.kind(), self.metadata.as_ref().unwrap().namespace(), self.metadata.as_ref().unwrap().name())
    }
    fn update(&self) { 
        println!("update for {} {}/{}",self.kind(), self.metadata.as_ref().unwrap().namespace(), self.metadata.as_ref().unwrap().name())
    }
    fn delete(&self) { 
        println!("delete for {} {}/{}",self.kind(), self.metadata.as_ref().unwrap().namespace(), self.metadata.as_ref().unwrap().name())
    }
}

impl ProcessResource for v1alpha1::VirtualRouter {
    fn kind(&self) -> String { "VirtualRouter".to_string() }
    fn add(&self) { 
        println!("add for {} {}/{}",self.kind(), self.metadata.as_ref().unwrap().namespace(), self.metadata.as_ref().unwrap().name())
    }
    fn update(&self) { 
        println!("update for {} {}/{}",self.kind(), self.metadata.as_ref().unwrap().namespace(), self.metadata.as_ref().unwrap().name())
    }
    fn delete(&self) { 
        println!("delete for {} {}/{}",self.kind(), self.metadata.as_ref().unwrap().namespace(), self.metadata.as_ref().unwrap().name())
    }
}

impl ProcessResource for v1alpha1::VirtualMachineInterface {
    fn kind(&self) -> String { "VirtualMachineInterface".to_string() }
    fn add(&self) {
        println!("add for {} {}/{}",self.kind(), self.metadata.as_ref().unwrap().namespace(), self.metadata.as_ref().unwrap().name())
    }
    fn update(&self) { 
        println!("update for {} {}/{}",self.kind(), self.metadata.as_ref().unwrap().namespace(), self.metadata.as_ref().unwrap().name())
    }
    fn delete(&self) { 
        println!("delete for {} {}/{}",self.kind(), self.metadata.as_ref().unwrap().namespace(), self.metadata.as_ref().unwrap().name())
    }
}
