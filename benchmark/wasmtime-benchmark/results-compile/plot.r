#!/usr/bin/env Rscript

args = commandArgs(trailingOnly=TRUE)

if (length(args) == 0) {
    control <- "master"
    variable <- "integrate-peepmatic"
} else if (length(args) == 2) {
    control <- args[1]
    variable <- args[2]
} else {
    print("error: Must supply either 0 or 2 args.\n")
    quit()
}


library(ggplot2)

## TIME

data <- read.table("time.csv", header = TRUE, sep = ",", stringsAsFactors = TRUE)

controlTime <- subset(data, data$Branch == control)$ElapsedSeconds
print(paste("mean time control: ", mean(controlTime)))

variableTime <- subset(data, data$Branch == variable)$ElapsedSeconds
print(paste("mean time variable: ", mean(variableTime)))

walltime <- ggplot(data,
                  aes(x = data$Branch,
                      y = data$ElapsedSeconds,
                      color = data$Branch,
                      pch = data$Branch)) +
    geom_boxplot(outlier.shape = NA) +
    geom_jitter(position = position_jitter(width = 0.1)) +
    scale_y_continuous(limits = quantile(data$ElapsedSeconds, c(NA, 0.99))) +
    expand_limits(y = 0) +
    theme(legend.position = "none",
          axis.text.x = element_text(angle = 45, hjust = 1),
          axis.title.x = element_blank()) +
    ggtitle("Compiling and running `markdown.wasm '# Hello, World!'` without a code cache") +
    labs(y = "Elapsed Time (seconds)")

ggsave(plot = walltime,
       file = "time.png",
       device = "png")

controlRss <- subset(data, data$Branch == control)$RSS
print(paste("mean rss control: ", mean(controlRss)))

variableRss <- subset(data, data$Branch == variable)$RSS
print(paste("mean rss variable: ", mean(variableRss)))

maxRss <- ggplot(data,
                  aes(x = data$Branch,
                      y = data$RSS,
                      color = data$Branch,
                      pch = data$Branch)) +
    geom_boxplot(outlier.shape = NA) +
    geom_jitter(position = position_jitter(width = 0.1)) +
    scale_y_continuous(limits = quantile(data$RSS, c(NA, 0.99))) +
    expand_limits(y = 0) +
    theme(legend.position = "none",
          axis.text.x = element_text(angle = 45, hjust = 1),
          axis.title.x = element_blank()) +
    ggtitle("Compiling and running `markdown.wasm '# Hello, World!'` without a code cache") +
    labs(y = "Max RSS (kilobytes)")

ggsave(plot = maxRss,
       file = "max-rss.png",
       device = "png")

## PERF EVENTS

data <- read.table("perf_events.csv", header = TRUE, sep = ",", stringsAsFactors = TRUE)

cacheMisses <- subset(data, data$Event == "cache-misses")
branchMisses <- subset(data, data$Event == "branch-misses")
instructions <- subset(data, data$Event == "instructions")

controlCacheMisses <- subset(cacheMisses, cacheMisses$Branch == control)$Count
print(paste("mean cache misses control: ", mean(controlCacheMisses)))

controlBranchMisses <- subset(branchMisses, branchMisses$Branch == control)$Count
print(paste("mean branch misses control: ", mean(controlBranchMisses)))

controlInstructions <- subset(branchMisses, branchMisses$Branch == control)$Count
print(paste("mean instructions control: ", mean(controlInstructions)))

variableCacheMisses <- subset(cacheMisses, cacheMisses$Branch == variable)$Count
print(paste("mean cache misses variable: ", mean(variableCacheMisses)))

variableBranchMisses <- subset(branchMisses, branchMisses$Branch == variable)$Count
print(paste("mean branch misses variable: ", mean(variableBranchMisses)))

variableInstructions <- subset(branchMisses, branchMisses$Branch == variable)$Count
print(paste("mean instructions variable: ", mean(variableInstructions)))

cacheMissesPlot <- ggplot(cacheMisses,
                          aes(x = cacheMisses$Branch,
                              y = cacheMisses$Count,
                              color = cacheMisses$Branch,
                              pch = cacheMisses$Branch)) +
    geom_boxplot(outlier.shape = NA) +
    geom_jitter(position = position_jitter(width = 0.1)) +
    scale_y_continuous(limits = quantile(cacheMisses$Count, c(NA, 0.99))) +
    expand_limits(y = 0) +
    theme(legend.position = "none",
          axis.text.x = element_text(angle = 45, hjust = 1),
          axis.title.x = element_blank()) +
    ggtitle("Compiling and running `markdown.wasm '# Hello, World!'` without a code cache") +
    labs(y = "Cache Misses")

ggsave(plot = cacheMissesPlot,
       file = "cache-misses.png",
       device = "png")

branchMissesPlot <- ggplot(branchMisses,
                          aes(x = branchMisses$Branch,
                              y = branchMisses$Count,
                              color = branchMisses$Branch,
                              pch = branchMisses$Branch)) +
    geom_boxplot(outlier.shape = NA) +
    geom_jitter(position = position_jitter(width = 0.1)) +
    scale_y_continuous(limits = quantile(branchMisses$Count, c(NA, 0.99))) +
    expand_limits(y = 0) +
    theme(legend.position = "none",
          axis.text.x = element_text(angle = 45, hjust = 1),
          axis.title.x = element_blank()) +
    ggtitle("Compiling and running `markdown.wasm '# Hello, World!'` without a code cache") +
    labs(y = "Branch Misses")

ggsave(plot = branchMissesPlot,
       file = "branch-misses.png",
       device = "png")

instructionsPlot <- ggplot(instructions,
                           aes(x = instructions$Branch,
                               y = instructions$Count,
                               color = instructions$Branch,
                               pch = instructions$Branch)) +
    geom_boxplot(outlier.shape = NA) +
    geom_jitter(position = position_jitter(width = 0.1)) +
    scale_y_continuous(limits = quantile(instructions$Count, c(NA, 0.99))) +
    expand_limits(y = 0) +
    theme(legend.position = "none",
          axis.text.x = element_text(angle = 45, hjust = 1),
          axis.title.x = element_blank()) +
    ggtitle("Compiling and running `markdown.wasm '# Hello, World!'` without a code cache") +
    labs(y = "Instructions Retired")

ggsave(plot = instructionsPlot,
       file = "instructions.png",
       device = "png")

print("All done!\n")
