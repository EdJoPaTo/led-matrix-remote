#!/usr/bin/env bash

echo "# Print 'one' in color red, then wait a second"
echo "bri 40"
echo "sat 100"
echo "hue 0"
echo "text one"
sleep 1

echo "# Print 'two' in color green, then wait a second"
echo "hue 120"
echo "text two"
sleep 1

echo "# Print 'three' in color blue"
echo "hue 240"
echo "text three"
