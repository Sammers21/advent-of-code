module MyLib (calibrationValue) where
import GHC.Unicode (isDigit)
import Data.Foldable (minimumBy, maximumBy)
import Debug.Trace (trace)
import Control.Arrow (Arrow(second))
import Data.Char (digitToInt)


calibrationValue :: String -> Int
calibrationValue input = sum $ map calibrationValueOfLine $ lines input

calibrationValueOfLine :: String -> Int
calibrationValueOfLine line = trace ("Line " ++ line ++ " has a calibration value of " ++ show ( high * 10 + low)) (high * 10 + low)
  where
    low = findH line
    high = findL line


findL :: String -> Int
findL input = trace ("Minimum char in line " ++ input ++ " is " ++ show lowest) digitToInt lowest
    where
        lowest :: Char
        lowest = snd $ minimumBy (\(a, _) (b, _) -> compare a b) $ filter (\(_, a) -> isDigit a) $ indexAndChar input

findH :: String -> Int
findH input = trace ("Maximum char in line " ++ input ++ " is " ++ show maximum) digitToInt maximum
    where
        maximum :: Char
        maximum = snd $ maximumBy (\(a, _) (b, _) -> compare a b) $ filter (\(_, a) -> isDigit a) $ indexAndChar input

indexAndChar :: String -> [(Int, Char)]
indexAndChar str = zip [0..] str