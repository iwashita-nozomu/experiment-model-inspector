# Synthetic learning pipeline: referenced data does not exist and is never read.
samples = load_csv("data/synthetic.csv")
features = normalize(samples)
prediction = model(features, weights)
loss = mean((prediction - targets) ** 2)
score = evaluate(prediction, targets)
save_metrics(score, "results/metrics.json")
