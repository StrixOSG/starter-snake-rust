terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }

  required_version = ">= 1.2.0"
}

provider "aws" {
  region  = "us-west-2"
  profile = "matt.dev"
  access_key = var.AWS_ACCESS_KEY_ID
  secret_key = var.AWS_SECRET_ACCESS_KEY
}

resource "aws_instance" "battle_snakes_server" {
  ami = "ami-04e914639d0cca79a"
  instance_type = "t2.micro"
  key_name = "battle_snake"
  user_data = <<EOF
  #!/bin/bash
  echo "Copying the SSH Key to the server"
  echo -e "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAACAQDdjY6tCvdp8pkG+mTWfMaaRh3OjQ1pBuoBXIMFEtgwisNF1OwwhX2eH5cGHJV6kFmdHSUEO/gIY7/nN9vRwATFp07f2JE1+yEM8CnGy3hruF/ycbmdhm1Yg6X/WL8RuIBGVZVHTdAAIKV/zeOBzD2lPHc1nAuNDnhMroa203zSI6TfLbEVArTIsYJD719p8bMwxhoJC3+ERqBTJ2x0sL1pvIrFCgdJvkHw5fMjRe3qt4OthD61lz4aoDxe1PAJfhkTUQFiHKBN6m1URNkBmLxwAKa5GNQla0S3qnWbb9UJ4KjRRQk0c96kjF7e+yRlyWotIA0EVCyj9ZuzmoYrvBDPQNK/CLvFWKTLffVu47LGQrqrY70xDbtzxOzP20XrWGVrQt85C21LRAnRE9RhunU01kC7gex9WTTKE/SejdOaE88OszYUr8VZZCFkQ0d1oiDhq9c7a6oadbQO7gxoZQ34Hu4FR+1ecU/8Kse62UGdyuKwSQz84bie92efjzsL83BrKMai75tzXEGitw9WyF9aq0EXGkGIxR7r2zG65EhvjE6jM+3J/uqAwgBXvT1nD1VDiI/g+kVNAs6FXE3z+nu5bWP2RS9jCeLb9Zz8klZaBM/rh32uy56e8B4Zme6SeiOXk1EfslWXI95lVdYTVNlJ+vzvAFxOweJhRn8yh13c5Q== matthewhamilton@Matthews-MBP" >> /home/ubuntu/.ssh/authorized_keys
  sudo yum update -y
  sudo yum install ec2-instance-connect
  sudo yum install rust cargo
  sudo cargo run 
EOF
  tags = {
    Name = "BattleSnakes"
  }
}

variable "AWS_ACCESS_KEY_ID" {
    type = string
}

variable "AWS_SECRET_ACCESS_KEY" {
    type = string
}