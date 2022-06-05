func numUniqueEmails(emails []string) int {
	m := make(map[string]string)
	for _, v := range emails {
		idx := -1
		var builder strings.Builder
		ignore := false
		for i, c := range v {
			if c == '@' {
				idx = i
				break
			}
			if c == '.' {
				continue
			}
			if c == '+' {
				ignore = true
				continue
			}
			if !ignore {
				builder.WriteString(string(c))
			}
		}
		sub := v[idx:]
		builder.WriteString(sub)
		str := builder.String()
		m[str] = str
	}
	return len(m)
}
