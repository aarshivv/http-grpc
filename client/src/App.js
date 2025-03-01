import logo from './logo.svg';
import './App.css';

import React, { useState } from "react";
import { TaskRequest } from "./proto/task_service_pb.js";
import { TaskServiceClient } from "./proto/task_service_grpc_web_pb.js";

function App() {
    const [response, setResponse] = useState("");

    const callGRPC = () => {
        const client = new TaskServiceClient("https://localhost", null,  null);

        const request = new TaskRequest();

        request.setTaskType("example_task");
        request.setMessage("This is a test task");
    
        // Adding attributes
        const attributes = request.getAttributesMap();
        attributes.set("key1", "value1");
        attributes.set("key2", "value2");

        client.processTask(request, {"Authorization": "Bearer token"}, (err, res) => {
            if (err) {
                console.error("Error:", err.message);
                setResponse("Error: " + err.message);
            } else {
                console.log("Response:", res.getMessage());
                setResponse("Response: " + res.getMessage());
            }
        });
    };

    return (
        <div>
            <h1>gRPC-Web Client</h1>
            <button onClick={callGRPC}>Call gRPC</button>
            <p>{response}</p>
        </div>
    );
}

export default App;
