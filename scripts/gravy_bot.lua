local Direction = require("scripts/libs/direction")

local area_id = "default"

local bot_pos = Net.get_object_by_name(area_id, "Gravy Bot")

local gravy_bot_id = Net.create_bot({
  name = "Gravy Bot",
  area_id = area_id,
  texture_path = "/server/assets/prog.png",
  animation_path = "/server/assets/prog.animation",
  x = bot_pos.x,
  y = bot_pos.y,
  z = bot_pos.z,
  solid = true
})

local mug_texture_path = "resources/ow/prog/prog_mug.png"
local mug_animation_path = "resources/ow/prog/prog_mug.animation"

Net.set_bot_direction(gravy_bot_id, Direction.DOWN_RIGHT)

Net:on("actor_interaction", function(event)
    if event.actor_id ~= gravy_bot_id then
        return
    end
  local player_id = event.player_id

  Net.lock_player_input(player_id)

  local player_pos = Net.get_player_position(player_id)

  local face_direction = Direction.from_points(bot_pos, player_pos)

  Net.set_bot_direction(gravy_bot_id, face_direction)

  Net.message_player(player_id, "THIS IS A BOOKMARK TO GRAVY YUM.", mug_texture_path, mug_animation_path);
  Async.message_player(player_id, "MAYBE IT WILL HELP YOU GET AROUND!", mug_texture_path, mug_animation_path).and_then(function()
    Net.unlock_player_input(player_id)
    Net.set_bot_direction(gravy_bot_id, Direction.DOWN_RIGHT)
  end)
end)
