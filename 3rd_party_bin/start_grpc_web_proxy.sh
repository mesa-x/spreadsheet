#!/bin/env bash

./grpcwebproxy-v0.13.0-linux-x86_64 --backend_addr=localhost:50051 --backend_tls=false --run_tls_server=false --allow_all_origins
