var PROTO_PATH = __dirname + '/../proto/hello_world.proto';

var async = require('async');
var grpc = require('@grpc/grpc-js');
var protoLoader = require('@grpc/proto-loader');
var packageDefinition = protoLoader.loadSync(
  PROTO_PATH,
  {
    keepCase: true,
    longs: String,
    enums: String,
    defaults: true,
    oneofs: true
  });
var protoDescriptor = grpc.loadPackageDefinition(packageDefinition);
var helloworld = protoDescriptor.helloworld;
var client = new helloworld.Greeter('localhost:50051',
  grpc.credentials.createInsecure());


/**
 * @param {function():?} callback
 */
function runSayHello(callback) {
  client.sayHello({ name: 'dpp' }, {}, (err, response) => {
    if (err) {
      console.log("Err == ", err);

    } else {
      console.log("Response == ", response);
      console.log(response.message);
    }
    callback();
  });
}


/**
 * Run all of the demos in order
 */
function main() {
  async.series([
    runSayHello,
  ]);
}

if (require.main === module) {
  main();
}
