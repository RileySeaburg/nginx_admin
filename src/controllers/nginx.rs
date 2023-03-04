use std::{
    fs::{self, File},
    io::prelude::*,
    io::Write,
    process::{Command, Output},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Vhost {
    domain: String,
}

pub struct VhostHandler {
    vhosts: Vec<Vhost>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VhostResponse {
    pub message: String,
    pub error: Option<String>,
    pub status: Option<i32>,
}

// reload nginx using sudo, return the status of the reload as a string
pub fn reload_nginx() -> Result<Output, std::io::Error> {
    Command::new("sudo")
        .arg("nginx")
        .arg("-s")
        .arg("reload")
        .output()
}
pub fn stop_nginx() -> Result<Output, std::io::Error> {
    Command::new("sudo")
        .arg("nginx")
        .arg("-s")
        .arg("stop")
        .output()
}

pub extern fn start_nginx() -> Result<Output, std::io::Error> {
    Command::new("sudo")
        .arg("systemctl")
        .arg("start")
        .arg("nginx")
        .output()
}



impl VhostHandler {
    /// # Name: reload_nginx
    /// # Description: A controller to reload nginx

    // Create Vhost
    // Create a new vhost file in /etc/nginx/evolving_hosts/vhosts that ends in .conf
    // The function takes a domain name as a parameter and creates a new vhost file
    // with the domain name as the file name
    pub fn create_vhost_conf(domain: &str) -> Result<VhostResponse, std::io::Error> {
        std::fs::File::create(format!("/etc/nginx/evolving_hosts/vhosts/{}.conf", domain))?;

        let config = format!(
            r#"
            server {{
        listen 80;
        listen [::]:80;
        server_name {};
        root /var/www/html/{}/html/;
        index index.html;
        location / {{
            try_files $uri $uri/ =404;
        }}
    }}
    "#,
            domain, domain
        );

        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(format!("/etc/nginx/evolving_hosts/vhosts/{}.conf", domain))
            .unwrap();
        file.write_all(config.as_bytes()).unwrap_or_else(|e| {
            match e.kind() {
                std::io::ErrorKind::NotFound => VhostResponse {
                    message: format!("Error creating vhost for {}", domain),
                    error: Some("File not found".to_string()),
                    status: Some(404),
                },

                _ => VhostResponse {
                    message: format!("Vhost created for {}", domain),
                    error: None,
                    status: None,
                },
            };
        });
        // write the vhost file
        Ok(VhostResponse {
            message: format!("Vhost created for {}", domain),
            error: None,
            status: None,
        })
    }

    // Create Directory for vhost
    // Check if the directory exists
    // If it does not exist, create a new directory in /var/www/html//{domain} for the vhost
    // Create a new directory in /var/www/html//{domain} for the vhost
    // Set the permissions of the directory to 755
    pub fn create_vhost_directory(domain: &str) -> Result<VhostResponse, std::io::Error> {
        // set the permissions of /var/www/html/ to writeable
        Command::new("sudo")
            .arg("chmod")
            .arg("777")
            .arg("/var/www/html/")
            .output()
            .expect("Failed to set permissions for /var/www/html/");

        println!("Permissions set for /var/www/html/");
        // check if the directory exists and create it if it does not
        // std::fs::create_dir_all(format!("/var/www/html/{}/html/", domain)).unwrap_or_else(|e| {
        //     match e.kind() {
        //         std::io::ErrorKind::NotFound => VhostResponse {
        //             message: format!("Error creating directory for {}", domain),
        //             error: Some("File not found".to_string()),
        //             status: Some(404),
        //         },
        //         std::io::ErrorKind::PermissionDenied => VhostResponse {
        //             message: format!("Error creating directory for {}", domain),
        //             error: Some("Permission denied".to_string()),
        //             status: Some(403),
        //         },
        //         std::io::ErrorKind::AlreadyExists => VhostResponse {
        //             message: format!("Directory already exists for {}", domain),
        //             error: Some("Directory already exists".to_string()),
        //             status: Some(409),
        //         },

        //         _ => VhostResponse {
        //             message: format!("Directory created for {}", domain),
        //             error: None,
        //             status: None,
        //         },
        //     };
        // });

        Command::new("sudo")
            .arg("mkdir")
            .arg("-p")
            .arg(format!("/var/www/{}", domain))
            .output()
            .expect("Failed to create directory for /var/www/html/");

        Command::new("sudo")
            .arg("mkdir")
            .arg("-p")
            .arg(format!("/var/www/html/{}/html/", domain))
            .output()
            .expect("Failed to create directory for /var/www/html/domain/html");

        println!("Directory created for {}", domain);

        /// set the permissions of the directory so the user can write to it
        Command::new("sudo")
            .arg("chown")
            .arg("-R")
            .arg("evolvingadmin:evolvingadmin")
            .arg(format!("/var/www/html/{}/html/", domain))
            .output()
            .expect("Failed to set permissions for /var/www/html/");

       
        // Restart nginx
        Command::new("sudo")
            .arg("nginx")
            .arg("-s")
            .arg("reload")
            .output()
            .expect("Failed to reload nginx");

        Ok(VhostResponse {
            message: format!("Directory created for {}", domain),
            error: None,
            status: None,
        })
    }

    // Create Index.html
    // Create a new index.html file in /var/www/html//{domain} for the vhost
    // Set the permissions of the file to 644
    pub fn create_vhost_index(domain: &str) -> Result<VhostResponse, std::io::Error> {
        File::create(format!("/var/www/html/{}/html/index.html", domain))?;
        // Command::new("sudo")
        //     .arg("touch")
        //     .arg(format!("/var/www/html/{}/html/index.html", domain))
        //     .output()
        //     .expect("Failed to create index.html");

        // std::fs::File::create(format!("/var/www/html/{}/html/index.html", domain))?;

        // define content for index.html
        let content = r#"
<Doctype html>
<html>
    <head>
        <title>{{domain}}</title>
    </head>
    <body>
        <h1>{{domain}}</h1>
    </body>
</html>
"#;

        // Write to Index.html
        // Write to the index.html file in /var/www/html//{domain} for the vhost

        fs::write(
            format!("/var/www/html/{}/html/index.html", domain),
            content.replace("{{domain}}", domain),
        )?;

        Ok(VhostResponse {
            message: format!("Created index.html for {}", domain),
            error: None,
            status: None,
        })

        // // set the permissions of the file to 644
        // Command::new("sudo")
        //     .arg("chmod")
        //     .arg("644")
        //     .arg(format!("/var/www/html/{}/html/index.html", domain))
        //     .output()
    }

    // Delete vhost file
    // Delete the vhost file in /etc/nginx/evolving_hosts/vhosts/{domain}.conf
    pub fn delete_vhost_file(domain: &str) -> Result<Output, std::io::Error> {
        std::process::Command::new("sudo")
            .arg("rm")
            .arg(format!("/etc/nginx/evolving_hosts/vhosts/{}.conf", domain))
            .output()
    }

    // Delete vhost directory
    // Delete the vhost directory in /var/www/html//{domain}
    pub fn delete_vhost_directory(domain: &str) -> Result<Output, std::io::Error> {
        std::process::Command::new("sudo")
            .arg("rm")
            .arg("-r")
            .arg(format!("/var/www/html/{}/html/", domain))
            .output()
    }
    pub fn create_vhost(domain: String) -> Result<VhostResponse, std::io::Error> {
        Self::create_vhost_conf(&domain).unwrap_or_else(|e| match e.kind() {
            std::io::ErrorKind::AlreadyExists => VhostResponse {
                message: format!("Vhost {} already exists", domain),
                error: None,
                status: None,
            },
            _ => VhostResponse {
                message: format!("Vhost {} created", domain),
                error: None,
                status: None,
            },
        });
        Self::create_vhost_directory(&domain).unwrap_or_else(|e| match e.kind() {
            std::io::ErrorKind::AlreadyExists => VhostResponse {
                message: format!("Vhost {} already exists", domain),
                error: None,
                status: None,
            },
            std::io::ErrorKind::NotFound => VhostResponse {
                message: format!("Error creating vhost for {}", domain),
                error: Some("File not found".to_string()),
                status: Some(404),
            },
            std::io::ErrorKind::PermissionDenied => VhostResponse {
                message: format!("Error creating vhost for {}", domain),
                error: Some("Permission denied".to_string()),
                status: Some(403),
            },
            _ => VhostResponse {
                message: format!("Vhost {} created test", domain),
                error: None,
                status: None,
            },
        });
        Self::create_vhost_index(&domain).unwrap_or_else(|e| match e.kind() {
            std::io::ErrorKind::AlreadyExists => VhostResponse {
                message: format!("Vhost {} already exists", domain),
                error: None,
                status: None,
            },
            std::io::ErrorKind::NotFound => VhostResponse {
                message: format!("Error creating vhost for {}", domain),
                error: Some("File not found".to_string()),
                status: Some(404),
            },
            std::io::ErrorKind::PermissionDenied => VhostResponse {
                message: format!("Error creating vhost for {}", domain),
                error: Some("Permission denied".to_string()),
                status: Some(403),
            },
            _ => VhostResponse {
                message: format!("Vhost index file created {} created yes", domain),
                error: None,
                status: None,
            },
        });
        reload_nginx()?;

        Ok(VhostResponse {
            message: format!("Vhost {} created", domain),
            error: None,
            status: None,
        })
    }

    /// # Name: 
    ///     get_vhost
    /// ## Description: 
    ///     Get the contents of a vhost file in /etc/nginx/evolving_hosts/vhosts/{domain}.conf
    /// ## Arguments
    ///     domain: String
    /// ## Returns:
    ///     Result<VhostResponse, std::io::Error>`
    /// ## Errors: 
    ///     std::io::Error
    /// ### - Example: 
    ///      get_vhost("example.com")
    /// ### - Example: 
    ///      get_vhost("example.com").unwrap().message
    pub fn get_vhost(domain: String) -> Result<VhostResponse, std::io::Error> {
        let contents = fs::read_to_string(format!("/etc/nginx/evolving_hosts/vhosts/{}.conf", domain))?;
        Ok(VhostResponse {
            message: contents,
            error: None,
            status: None,
        })
    }

    
}
