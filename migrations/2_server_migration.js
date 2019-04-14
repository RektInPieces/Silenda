const Server = artifacts.require("GameServerContract");

module.exports = function(deployer) {
  deployer.deploy(Server);
};
