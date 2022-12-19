variable "environment" {
  type        = string
  description = "Select an environment tag, typically dev/prod"
}

variable "domain" {
  type        = string
  description = "Nominate the domain name declared in route53 to use, such as domain.com"
}

variable "subdomains" {
  type        = list(string)
  description = "List of URL associated with the root domain from which to serve the application, ie ['domain.com', 'webrtc.domain.com']"
}

variable "contact" {
  type        = string
  description = "Email address to contact for any issue"
}

