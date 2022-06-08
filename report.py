#!/usr/bin/env python3

from subprocess import run

roms = [
    "Action_JunoFirst_JunoFirst.V1.00.hex",
    "Arcade%2FLasers%2F9489704cbaedb8fbfff8986d63eae61bdb0915d5.hex",
    "hollow_v0.31.hex",
    "Platformer%2FCastleBoy%2FCastleBoy.hex",
    "Puzzle_tinytris_TinyTris.hex",
    "Shooter_StarduinoTurob_game.hex",
    "SRN_AB_v122.hex",
    "unicorn_dash.hex",
]


run(['cargo', 'build', '--release', '--bin', 'report'], check=True, capture_output=True)

for rom in roms:
    rom_file = 'roms/' + rom
    replay_file = 'records/' + rom + '.txt'
    run(['cargo', 'run', '--release', '--bin', 'report', '--',
        rom_file,
        '--frames_screenshot', '600',
        '--replay_file', replay_file,
        '--seconds', '60',
        'reports',
        ], check=True)
