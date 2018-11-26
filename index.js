const Discord = require('discord.js');
const client = new Discord.Client();
const fs = require('fs');

client.on('ready', function(){
  console.log("Connected!");
});

client.on('message', (msg) => {
  if (msg.channel.name == 'testfulgurobot') {
    if (msg.content === 'ping') {
      msg.reply('pong');
    }
    if (msg.content === '!fulgurobot') {
      msg.reply("Commandes pour parier :\n!noir x -> parie x coquillages sur noir\n!blanc x -> parie x coquillages sur blanc\n!coq -> envoie en message privé votre nombre de coquillages");
    }
  }
});

async function login() {
  try {
    var oath;
    await fs.readFile('./oath2', (err, data) => {
      if (err) throw err;
      oath = data;
    });
    client.login(oath).catch();
  }
  catch(err) {
    throw err;
  }
}
login();
