package main

import (
  // "fmt"
  "io/ioutil"
	"log"
  "os"
  "os/signal"
  "strings"
  "syscall"
  "github.com/bwmarrin/discordgo"
)

func main()  {
  data, err := ioutil.ReadFile("auth")
	if err != nil {
		log.Fatalf("Error reading token: %v", err)
	}
	token := string(data[:len(data)-1])
	bot, err := discordgo.New("Bot " + token)
	if err != nil {
		log.Fatalf("couldn't connect do Discord: %v", err)
	}
  defer bot.Close()

  bot.AddHandler(messageHandler)
  err = bot.Open()
  if err != nil {
    log.Fatalf("couldn't listen to Discord")
  }
  guilds, err := bot.UserGuilds(10, "", "")
	if err != nil {
		log.Fatalf("couldn't get guilds from bot: %v", err)
	}

  for i := 0; i < len(guilds); i++ {
		if guilds[i].Name == "FulguroGo" {
			guildID := guilds[i].ID
			guild, err := bot.Guild(guildID)
			if err != nil {
				log.Fatalf("Couldn't get Ensicoin Guild from bot: %v", err)
			}

			for j := 0; j < len(guild.Channels); j++ {
				if guild.Channels[j].Name == "testfulgurobot" {
					bot.ChannelMessageSend(guild.Channels[j].ID, "Hello I'm working!")
          break
				}
			}
		}
    break
	}

  sc := make(chan os.Signal, 1)
	signal.Notify(sc, syscall.SIGINT, syscall.SIGTERM, os.Interrupt, os.Kill)
	<-sc

}

func messageHandler(s *discordgo.Session, m *discordgo.MessageCreate) {
  if m.Author.ID == s.State.User.ID {
		return
	}

	channel, err := s.Channel(m.ChannelID)
	if err != nil {
		log.Fatalf("error fetching the discord channel of a message: %v", err)
	}

	if channel.Name != "testfulgurobot" {
		return
	}

  content := m.Content

  if strings.EqualFold(content, "!fulgurobot") {
    s.ChannelMessageSend(m.ChannelID, "Je suis Fulgurobot !")
  }
  command := strings.Fields(content)[0]
  switch command {
  case "!blanc":

  }
}
