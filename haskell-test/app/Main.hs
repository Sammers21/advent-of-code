module Main where

import qualified MyLib (calibrationValue)

main :: IO ()
main = do
  test <- readFile "../data/day1p1.txt"
  putStrLn ("Test: " ++ (show (MyLib.calibrationValue test)))

