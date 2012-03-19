/* action_system.cpp

   Copyright (C) 2012 Risto Saarelma

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

#include "action_system.hpp"
#include <world/rules.hpp>
#include <world/parts.hpp>
#include <world/effects.hpp>

// TODO: into Msg_System
void msg(const char* fmt) {
  raw_msg(fmt);
}

bool Action_System::walk(Entity entity, const Vec2i& dir) {
  auto loc = spatial.location(entity);
  auto new_loc = loc + dir;
  if (spatial.can_pop(entity, new_loc)) {
    for (auto a : spatial.entities_on(spatial.footprint(entity, new_loc))) {
      if (a == entity) continue;
      // Uncrushable entities in the target area, abort movement.
      if (blocks_movement(a) && !can_crush(entity, a))
        return false;
    }

    spatial.push(entity);

    // XXX Hacky. Player is tracked by the view space object.
    if (entity == get_player())
      fov.move_pos(dir);

    for (auto a : spatial.entities_on(spatial.footprint(entity, new_loc))) {
      if (blocks_movement(a)) {
        // Crushing damages you.
        damage(entity, entities.as<Blob_Part>(a).armor / 2);
        msg("Crush!");
        entities.destroy(a);
      }
    }
    spatial.pop(entity, new_loc);
    // Energy cost for movement.
    // TODO: account for terrain differences.
    entities.as<Blob_Part>(entity).energy -= 100;
    return true;
  } else {
    return false;
  }
}

bool Action_System::shoot(Entity entity, const Vec2i& dir) {
  ASSERT(is_hex_dir(dir));
  // TODO: Entities have multiple weapons. (The weapon could be the entity though.)
  const int range = 6; // TODO: Entities have different fire ranges.
  int dist = 0;
  Location loc = spatial.location(entity);

  for (loc = loc + dir; dist < range; loc = loc + dir) {
    dist++;

    bool hit_entity = false;
    for (auto& a : spatial.entities_at(loc)) {
      if (a != entity) {
        hit_entity = true;
        break;
      }
    }

    if (hit_entity) {
      msg("Zap!");
      damage(loc, entities.as<Blob_Part>(entity).damage);
      break;
    }
    if (terrain.blocks_shot(loc))
      break;
  }

  beam_fx(spatial.location(entity), dir, dist, Color("pink"));

  auto& blob = entities.as<Blob_Part>(entity);
  // Energy cost for shooting.
  blob.energy -= 100;
}

void Action_System::damage(Location location, int amount) {
  for (auto a : spatial.entities_at(location))
    damage(a, amount);
}

void Action_System::damage(Entity entity, int amount) {
  if (entities.has(entity, Blob_Kind)) {
    auto& blob = entities.as<Blob_Part>(entity);
    blob.armor -= amount;
    if (blob.armor <= 0) {
      explosion_fx(spatial.location(entity));
      entities.destroy(entity);
    }
  }
}

bool Action_System::is_ready(Entity entity) {
  try {
    return entities.as<Blob_Part>(entity).energy >= 0;
  } catch (Part_Not_Found& e) {
    return false;
  }
}

bool Action_System::can_crush(Entity entity, Entity crushee) {
  return entities.as<Blob_Part>(entity).big &&
    !entities.as<Blob_Part>(crushee).big;
}

bool Action_System::blocks_movement(Entity entity) {
  return entities.has(entity, Blob_Kind);
}

Entity Action_System::active_entity() {
  return entities.entity_after(previous_entity);
}

void Action_System::next_entity() {
  previous_entity = entities.entity_after(previous_entity);

  try {
    start_turn_update(active_entity());
  } catch (Entity_Not_Found &e) {}
}

void Action_System::start_turn_update(Entity entity) {
  try {
    auto& blob = entities.as<Blob_Part>(entity);
    blob.energy += blob.power;
  } catch (Part_Not_Found& e) {}
}
