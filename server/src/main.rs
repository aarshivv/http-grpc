use std::net::SocketAddr;
use protos::policy_service::{task_service_server::{TaskService, TaskServiceServer}, TaskRequest, TaskResponse};
use tonic::{transport::Server, Request, Response, Status};

// Service exposed for reloading casbin policy
pub mod protos {
    pub mod policy_service {
        tonic::include_proto!("taskservice");
    }

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("taskservice_descriptor");
}

#[derive(Debug)]
pub struct ExecuteTaskService;


#[tonic::async_trait]
impl TaskService for ExecuteTaskService {
    async fn process_task(
        &self,
        request: Request<TaskRequest>,
    ) -> Result<Response<TaskResponse>, Status> {
        let request = request.into_inner();
        println!("Request: {request:?}");

        let assert_val = "some_value";
        if request.task_type.ne(assert_val) {
            Err(Status::permission_denied("PERMISSION DENIED"))
        }
        else {
            Ok(Response::new(TaskResponse {
                is_success: true,
                message: String::from("Policy reloaded successfully"),
            }))
        }
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let reload_policy = ExecuteTaskService;

    let addr: SocketAddr = "0.0.0.0:6000".parse().unwrap();
    println!("AuthorizationServer listening on {}", addr);

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(protos::FILE_DESCRIPTOR_SET)
        .build_v1()
        .unwrap();

    Server::builder()
        .add_service(TaskServiceServer::new(reload_policy))
        .add_service(reflection_service)
        .serve(addr)
        .await?;

    Ok(())
}
