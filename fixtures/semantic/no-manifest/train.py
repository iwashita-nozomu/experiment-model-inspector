# Synthetic source for host-GPT interpretation; never executed by the inspector.
samples = load_csv("data/synthetic.csv")
features = center(samples)
prediction = linear(features, weights)
residual = prediction - targets
loss = mean(residual * residual) + penalty * sum(weights * weights)
score = mean(abs(residual))
save_metrics(score, "results/metrics.json")
