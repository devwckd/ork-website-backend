# ork

Ork aims to be a full-fledged pay-as-you-go Minecraft hosting suite built on top of kubernetes.

## ork-website-backend

`ork-website-backend` gives functionality to the website.  

Implemented/underway feature list:

| name                 | description                                                                                     |
|----------------------|-------------------------------------------------------------------------------------------------|
| auth                 | authenticates and authorizes website users                                                      |
| organization         | store and manage members and common configurations of a network                                 |
| deployment templates | defines deployment templates (e.g. minecraft proxies, servers, databases, ***custom services**) |
| deployments          | deploys and serves information of pods based on the given template                              |


***custom services**: [ork-bridge-service](https://github.com/devwckd/ork-bridge-service) is an example of a custom service provided by ork.