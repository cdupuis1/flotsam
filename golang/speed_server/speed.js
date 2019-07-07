//
// speed.js
//
// Javascript code to calculate transfer rate of binary data over http

// Globals
var t0 = 0;
var t1 = 0;
var num_times = 0;
var total_bandwidth = 0;
// Used to calculate percentage for progress bar
var kb_scale = 50000;

// Respond handler for http request.  Does the actual calculations for
// transfer speed and sets off another http request if one is needed.
function DataLoad() {
    t1 = performance.now()
    var time_in_sec = (t1 - t0) / 1000;
    var logger = document.getElementById("logger");
    var elem = document.getElementById("Progress");

    console.log("timer_in_sec=" + time_in_sec);
    console.log("size_in_kb=" + size_in_kb.value);
    var transfer_speed = size_in_kb.value / time_in_sec;
    console.log("transfer_speed=" + transfer_speed.toFixed(2));
    var percentage = transfer_speed / kb_scale;
    percentage = (percentage * 100).toFixed(0);
    console.log("percentage=" + percentage);

    if (num_times < total_num.value) {
        console.log("num_times=" + num_times);
        num_times++;
        total_bandwidth += transfer_speed;
        Progress.style.width = percentage + "%";
        setTimeout(GetData, 0);
    } else {
        total_bandwidth = total_bandwidth / num_times;
        logger.innerHTML = "<strong>Total speed:</strong> " + total_bandwidth.toFixed(2) + " KB/s";
        t0 = 0;
        t1 = 0;
        num_times = 0;
        total_bandwidth = 0;
    }
}

// Initiates a http request based on the fields of the form
function GetData() {
    var logger = document.getElementById("logger");
    var size_in_kb = document.getElementById("size_in_kb");
    var total_num = document.getElementById("total_num");
    var site_name = document.getElementById("site_name");
    var xmlhttp = new XMLHttpRequest();

    xmlhttp.addEventListener("load", DataLoad);
    xmlhttp.open("GET", site_name.value + "/?size_in_kb=" + size_in_kb.value);
    t0 = performance.now();
    xmlhttp.send();
}