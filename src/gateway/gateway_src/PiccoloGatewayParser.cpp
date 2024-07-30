// SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
//
// SPDX-License-Identifier: Apache-2.0

#include <iostream>
#include <sstream>
#include <cstdlib>
#include "PiccoloGatewayParser.h"

void PiccoloGatewayParser::parse(PiccoloEvent* pe)
{
	std::cout << "parser start" << std::endl;

	const char* env_ip = std::getenv("HOST_IP");

	std::string etcdAddr;
	if(env_ip == nullptr)
	{
		etcdAddr = std::string("0.0.0.0").append(":2379");
	}else
	{
		etcdAddr = std::string(env_ip).append(":2379");
	}

	etcd::Client etcd(etcdAddr);
	std::string conditions = std::string(pe->name).append("/conditions");
	//std::string action = std::string(pe->name).append("/action");
	std::string action = std::string(pe->name);
	pe->actionKey = action;
	etcd::Response conditionsResponse = etcd.get(conditions).get();

	try {
		std::istringstream cs(conditionsResponse.value().as_string());
		YAML::Node receivedConditionsEvent = YAML::Load(cs);
		pe->express = receivedConditionsEvent["express"].as<std::string>();
		pe->targetValue = receivedConditionsEvent["value"].as<std::string>();
		pe->topic = receivedConditionsEvent["operands"]["value"].as<std::string>();
	} catch(YAML::ParserException& e) {
		std::cout << e.what() << std::endl;
	}

}
