package twofer

func ShareWith(name string) string {
	if name != "" {
		return "One for " + name + ", one for me."
	}

	return "One for you, one for me."
}
