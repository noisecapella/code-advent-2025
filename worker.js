import { run_worker } from './pkg';

console.log("worker.js loaded", self)

addEventListener("message", function(event) {
    console.log("received", event)
    run_worker(event, postMessage);
})
