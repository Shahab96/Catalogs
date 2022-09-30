data "google_dns_managed_zone" "this" {
  name = "dogar-dev"
}

resource "aws_route53_zone" "this" {
  name = "${terraform.workspace}.dogar.dev"
}

resource "google_dns_record_set" "this" {
  name         = "${aws_route53_zone.this.name}."
  managed_zone = data.google_dns_managed_zone.this.name
  type         = "NS"
  ttl          = 86400
  rrdatas      = [for name_server in aws_route53_zone.this.name_servers : "${name_server}."]
}

resource "aws_route53_record" "this" {
  for_each = {
    user       = module.user_service.zone
    rule       = module.rule_service.zone
    extraction = module.extraction_service.zone
  }

  name    = each.value.name
  ttl     = 86400
  type    = "NS"
  zone_id = aws_route53_zone.this.zone_id
  records = each.value.name_servers
}
