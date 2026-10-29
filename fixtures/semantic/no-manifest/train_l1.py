# Synthetic counterexample: a different objective, with the same surrounding calls.
samples = load_csv("data/synthetic.csv")
features = center(samples)
prediction = linear(features, weights)
residual = prediction - targets
loss = mean(abs(residual)) + penalty * sum(weights * weights)
score = mean(abs(residual))
save_metrics(score, "results/metrics.json")
