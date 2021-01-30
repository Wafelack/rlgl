local discordia = require('discordia')
local links = require('links')
local token = require('token')
local PREFIX = "~"
local client = discordia.Client()

local function split (inputstr, sep)
        if sep == nil then
                sep = "%s"
        end
        local t={}
        for str in string.gmatch(inputstr, "([^"..sep.."]+)") do
                table.insert(t, str)
        end
        return t
end 

local function create_embed(lang)
  local embed = {}
  embed.title = lang:gsub("%l", string.upper)
  if lang == "python" then
    embed.description = "Le python est le langage le plus recommandé pour démarrer la programmation."
  else
    embed.description = ""
  end

  embed.fields = {
    {
      name = "Cours",
      value = "\n" .. table.concat(links[lang], "\n\n"),
      inline = false,
    }
  }
  embed.footer = {
    text = PREFIX .. "cours " .. lang .. "• DevBot • Inspired from https://learndev.info"
  }
  embed.color = 0x4444EE

  return embed

end


local function get_lines(file)
  lines = {}
  for line in io.lines(file) do
    lines[#lines + 1] = line
  end
  return lines
end

client:on('ready', function()
    print('Logged in as ' .. client.user.username)
  end)

client:on('messageCreate', function(message)
   local splited = split(message.content, ' ')
   if splited[1] == PREFIX .. "cours" then
      if splited[2] ~= nil then     
         if links[splited[2]] == nil then
           message:reply("Invalid language.")
         else
           message:reply{embed = create_embed(splited[2])}
         end
      else
        message:reply("Please specify a language.")
      end

    elseif splited[1] == PREFIX .. 'tutos' then

      embed = {}

      embed.title = "Les tutos ... une si bonne chose ?"
      embed.description = "• [Pourquoi les tutos ne t'aident pas](https://practicalprogramming.fr/pourquoi-les-tutos-ne-taident-pas)"

      embed.footer = {
        text = PREFIX .. 'tutos • DevBot • Wafelack'
      }

      message:reply {
        embed = embed
      }

    elseif splited[1] == PREFIX .. 'presentation' and message.author.id == "723862906755743804" then

      embed = {}

      embed.title = "Salut, je suis **DevBot**"
      embed.description = "**PRÉFIXE:** `" .. PREFIX .. "`"
      embed.fields = {
        {
           name = "Me trouver",
           value = "Je suis développé via la bibliothèque [Discordia](https://github.com/SinisterRectus/discordia) et suis disponible sur [GitHub](https://github.com/wafelack/devbot) sous la license MPL-2.0.",
           inline = false
        },
        {
           name = "Bien démarrer",
           value = "Pour obtenir la liste de mes commandes, utilisez `" .. PREFIX .. "help`.",
           inline = false
        }
      }

      embed.footer = {
        text = PREFIX .. "presentation • DevBot • Wafelack"
      }

      embed.color = 0xFF8800

      message:reply {
        embed = embed
      }

    elseif splited[1] == PREFIX .. 'help' then

      embed = {}

      embed.title = "Aide"
      embed.description = "• **" .. PREFIX .. "ask**: Affiche la méthode de demande d'aide.\n• **" .. PREFIX .. "code**: Affiche la méthode de mise en forme du code.\n• **" .. PREFIX .. "cours <langage>**: Affiche la liste des cours pour un langage.\n• **".. PREFIX .. "help**: Affiche ce message."
      embed.footer = {
        text = PREFIX .. "help • DevBot • Wafelack"
      }
      embed.color = 0xFFFF00

      message:reply {
        embed = embed
      }

    elseif splited[1] == PREFIX .. 'code' then

      embed = {}
      embed.title = "Mettre en forme du code sur discord"
      embed.description = table.concat(get_lines("code.txt"), "\n")
      embed.footer = {
        text = PREFIX .. "code • DevBot • Wafelack"
      }
      embed.color = 0x444444

      message:reply{
        embed = embed
      }

    elseif splited[1] == PREFIX .. 'ask' then
      embed = {}

      embed.title = "Comment poser une question ?"
      embed.description = "Ce message a pour but de vous montrer les meilleurs moyens de poser une question"

      embed.fields = {
        {
          name = "Qui s'y connait en #{technologie} ?",
          value = "Ce genre de messages est contre-productif et n'incite pas les gens à vous aider, poser votre question directement vous apportera plus d'aide.\n\n>> https://dontasktoask.com",
          inline = false
        },
        {
          name = "Poser une question efficacement",
          value = "Pour poser une question efficacement, veillez à fournir:\n\n• Votre code\n• Vos potentielles erreurs\n• Le résultat obtenu\n• Le résultat attendu\n• Tout autre chose pouvant permettre de mieux vous aider.",
          inline = false
        },
        {
          name = "Chercher avant de demander",
          value = "Un minimum de recherches sur votre problème, sur la doc, google, stackoverflow, etc, est demandé afin de ne pas spammer les canaux de demandes trouvables en premier résultat de google.",
          inline = false
        }
      }
      embed.footer = {
        text = PREFIX .. "ask • DevBot • Wafelack"
      }
      embed.color = 0xFF8800

      message:reply {
        embed = embed
      }

    end

 end)

client:run("Bot " .. token)
