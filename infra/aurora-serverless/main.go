package main

import (
	"os"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ec2"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/rds"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func vpc(ctx *pulumi.Context) (*ec2.Vpc, error) {

	return ec2.NewVpc(ctx, "writeonce-vpc", &ec2.VpcArgs{
		CidrBlock:       pulumi.String("10.0.0.0/16"),
		InstanceTenancy: pulumi.String("default"),
		Tags: pulumi.StringMap{
			"dbFor": pulumi.String("blogs"),
		},
	})
}

func securityGroup(ctx *pulumi.Context, vpc *ec2.Vpc) (*ec2.SecurityGroup, error) {

	return ec2.NewSecurityGroup(ctx, "blog-db-sg", &ec2.SecurityGroupArgs{
		VpcId:       vpc.ID(),
		Description: pulumi.String("Allow DB access"),
		Ingress: ec2.SecurityGroupIngressArray{
			&ec2.SecurityGroupIngressArgs{
				Protocol:   pulumi.String("tcp"),
				FromPort:   pulumi.Int(5432),
				ToPort:     pulumi.Int(5432),
				CidrBlocks: pulumi.StringArray{vpc.CidrBlock},
			},
		},
		Egress: ec2.SecurityGroupEgressArray{
			&ec2.SecurityGroupEgressArgs{
				Protocol:   pulumi.String("-1"),
				FromPort:   pulumi.Int(0),
				ToPort:     pulumi.Int(0),
				CidrBlocks: pulumi.StringArray{pulumi.String("0.0.0.0/0")},
			},
		},
		Tags: pulumi.StringMap{
			"Name": pulumi.String("blog-db-sg"),
		},
	})
}

func subnetA(ctx *pulumi.Context, vpc *ec2.Vpc) (*ec2.Subnet, error) {

	return ec2.NewSubnet(ctx, "aurora-subnet-a", &ec2.SubnetArgs{
		VpcId:            vpc.ID(),
		CidrBlock:        pulumi.String("10.0.1.0/24"),
		AvailabilityZone: pulumi.String("eu-central-1a"),
		Tags: pulumi.StringMap{
			"Name": pulumi.String("aurora-subnet-a"),
		},
	})
}
func subnetB(ctx *pulumi.Context, vpc *ec2.Vpc) (*ec2.Subnet, error) {

	return ec2.NewSubnet(ctx, "aurora-subnet-b", &ec2.SubnetArgs{
		VpcId:            vpc.ID(),
		CidrBlock:        pulumi.String("10.0.2.0/24"),
		AvailabilityZone: pulumi.String("eu-central-1b"),
		Tags: pulumi.StringMap{
			"Name": pulumi.String("aurora-subnet-b"),
		},
	})
}

func cluster(ctx *pulumi.Context, sg *ec2.SecurityGroup, subnets []*ec2.Subnet) (*rds.Cluster, error) {
	subnetIds := pulumi.StringArray{}
	for _, s := range subnets {
		subnetIds = append(subnetIds, s.ID())
	}

	subnetGroup, err := rds.NewSubnetGroup(ctx, "aurora-subnet-group", &rds.SubnetGroupArgs{
		SubnetIds: subnetIds,
		Tags: pulumi.StringMap{
			"Name": pulumi.String("aurora-db-subnet-group"),
		},
	})
	if err != nil {
		return nil, err
	}

	result, err := rds.NewCluster(ctx, "postgresql", &rds.ClusterArgs{
		ClusterIdentifier: pulumi.String("aurora-cluster"),
		Engine:            pulumi.String(rds.EngineTypeAuroraPostgresql),
		EngineMode:        pulumi.String(rds.EngineModeProvisioned),
		EngineVersion:     pulumi.String("16.6"),
		DatabaseName:      pulumi.String("writeonce"),
		MasterUsername:    pulumi.String("writeonce"),
		MasterPassword:    pulumi.String("writeonce"),
		VpcSecurityGroupIds: pulumi.StringArray{
			sg.ID().ToStringOutput(),
		},
		DbSubnetGroupName: subnetGroup.Name,
		StorageEncrypted:  pulumi.Bool(true),
		Serverlessv2ScalingConfiguration: &rds.ClusterServerlessv2ScalingConfigurationArgs{
			MaxCapacity:           pulumi.Float64(1),
			MinCapacity:           pulumi.Float64(0),
			SecondsUntilAutoPause: pulumi.Int(3600),
		},
		SkipFinalSnapshot: pulumi.Bool(true),
	})
	if err != nil {
		return nil, err
	}
	_, err = rds.NewClusterInstance(ctx, "postgresql-instance", &rds.ClusterInstanceArgs{
		ClusterIdentifier: result.ID(),
		InstanceClass:     pulumi.String("db.serverless"),
		Engine:            result.Engine,
		EngineVersion:     result.EngineVersion,
	})
	if err != nil {
		return nil, err
	}
	return result, nil

}

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		deploymentType := os.Getenv("DEPLOYMENT_TYPE")

		if deploymentType == "aurora" {
			vpc, err := vpc(ctx)
			if err != nil {
				return err
			}

			sg, err := securityGroup(ctx, vpc)
			if err != nil {
				return err
			}

			subA, err := subnetA(ctx, vpc)
			if err != nil {
				return err
			}
			subB, err := subnetB(ctx, vpc)
			if err != nil {
				return err
			}

			cluster, err := cluster(ctx, sg, []*ec2.Subnet{subA, subB})
			if err != nil {
				return err
			}

			ctx.Export("vpcId", vpc.ID())
			ctx.Export("securityGroupId", sg.ID())
			ctx.Export("clusterEndpoint", cluster.Endpoint)

			return nil
		} else if deploymentType == "lambda" {
			// Deploy Lambda
			_, err := lambdaFunction(ctx)
			if err != nil {
				return err
			}

			ctx.Export("lambdaFunction", pulumi.String("example-lambda"))
		}
		return nil

	})
}
