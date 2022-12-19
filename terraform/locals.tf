locals {
  application = "webrtc"
  all_tags = {
    Application = local.application
    Repository  = "p2p-exchange"
    Environment = var.environment
    Terraform   = true
    Contact     = var.contact
  }
}
