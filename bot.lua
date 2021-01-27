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
    text = "Inspired from https://learndev.info"
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
   
    elseif splited[1] == PREFIX .. 'code' then
      message:reply(
        table.concat(get_lines("code.txt"), "\n")
      )
    end

 end)

client:run("Bot " .. token)
