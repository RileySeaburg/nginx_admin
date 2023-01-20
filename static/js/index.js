const tailwind = require("tailwindcss");

// Rusty Road
class RustyRoad {
    constructor() {
        this.name = "nginx_admin";
        this.greet = function () {
            console.log(`Welcome to ${this.name} powered by Rusty Road`);
        }
    }
}

const rusty_road = new RustyRoad();
rusty_road.greet();

document.getElementById("reload-nginx").addEventListener("click", function () {
    fetch("/reload_nginx", {
        method: "GET",
        headers: {
            "Content-Type": "application/json"
        }
    })
        .then(response => response.json())
        .then(data => {
            console.log(data);
        })
        .catch(error => {
            console.error(error);
        });
});

document.getElementById("stop-nginx").addEventListener("click", function () {
    fetch("/stop_nginx", {
        method: "GET",
        headers: {
            "Content-Type": "application/json"
        }
    })
        .then(response => response.json())
        .then(data => {
            console.log(data);
        })
        .catch(error => {
            console.error(error);
        });
});

document.getElementById("start-nginx").addEventListener("click", function () {
    fetch("/start_nginx", {
        method: "GET",
        headers: {
            "Content-Type": "application/json"
        }
    })
        .then(response => response.json())
        .then(data => {
            console.log(data);
        })
        .catch(error => {
            console.error(error);
        });
});

document.getElementById("vhost-button").addEventListener("click", (e) => {
    e.preventDefault();
    // get the form data
    var formData = {
        'domain': document.getElementById('vhost-domain').value
    };

    // send the form data to /create_vhost/{domain}
    fetch('/create_vhost/' + formData.domain, {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json'
        }
    }).then((response) => {
        return response.json();
    }).then((data) => {
        console.log(data);
    });
});