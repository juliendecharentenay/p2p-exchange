/*
 * VPC network for lambda
 */
resource "aws_vpc" "main" {
  cidr_block = "10.0.0.0/24"
  tags = local.all_tags
}

/*
 * Private subnet A
 */
resource "aws_subnet" "privateA" {
  vpc_id = aws_vpc.main.id
  cidr_block = "10.0.0.0/25"
  availability_zone = "eu-west-1a"
  tags = local.all_tags
}

resource "aws_network_acl" "privateA" {
  vpc_id     = aws_vpc.main.id
  subnet_ids = [ aws_subnet.privateA.id ]
/*
  egress {
  }
  ingress {
  }
*/

  tags = local.all_tags
}


