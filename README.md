# Vimgine

*Until I figure out a better name*

## Overview

Vimgine is a vim-styled, command based, lightweight game engine built on Rust. It uses the Bevy framework and its ECS (Entity Component System) style.

## Goals

Vimgine is supposed to be best at specifically 3d games *at the moment* and will be designed for an easy drop in drop out testing style.

Vimgine puts all user designated code areas in the src/usr folder, which is then split into cmp (component) and sys (system) folders, as well as a setup.rs file for start configuration.

Users can easily utilize this structure as well as shell scripts to speed up the process of quickly adding new components, systems, and entities/startup behavior to their game.
