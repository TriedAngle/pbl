import os

const BaseDir = currentSourcePath.parentDir()
const AssetDir* = BaseDir / "../../assets"
const PlotsDir* = AssetDir / "plots" 
const DataSetsDir* = AssetDir / "datasets"
const DataFile* = DataSetsDir / "retention_time.hdf5"

const RemoveUnderScores* = true
