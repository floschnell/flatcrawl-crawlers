# flatcrawl-crawlers

This repository is part of my flatcrawl project. It contains a Rust implementation of crawlers/scrapers for different real estate websites. It will scan those websites in scheduled cycles and extract information on new flats. Those new flats are then parsed into a consistent layout. Finally they are sent away for further processing.

I chose Rust for this project, because I wanted to learn the language and also it seemed to be a good fit because of its capabilities like speed and thread safetiness.

## Infrastructure

Flats that are found by this tool and its set of crawlers will be transmitted via AMQP to a message broker (in my case RabbitMQ) to be picked up by different processors. Those processors can be found in their [own repository](https://github.com/floschnell/flatcrawl-processors) and can be anything from email notifications to instant messaging bots. Currently there's only an implementation of a telegram bot, but you could imagine all kinds of different services that will listen to the queue and push new flats to interested users.

## Setup & Requirements

The application can be setup easily, all you will have to do is to copy the `config.sample.toml` to a file called `config.toml`. Now you can edit the settings within the file. The `thread_count` will specify how many threads will be used for the different crawlers and indirectly how many TCP connections will be created in parallel. The amqp section defines the endpoint where the message broker can be found. I simply ran [an existing docker image](https://hub.docker.com/_/rabbitmq/) on my domain with some PLAIN authetication.

To actually run the application on your machine, you will need to compile it first. Installing Rust is quite easy, find the instructions on [their website](https://www.rust-lang.org/en-US/install.html).

## Run

Once Rust is installed and the program is configured via the `config.toml`, you can start it up via
```
cargo run
```
On the first run it will download and compile all the dependencies as well. This might take up to a few minutes even.