# uniq

       Filter adjacent matching lines from INPUT (or standard input),
       writing to OUTPUT (or standard output).

       With no options, matching lines are merged to the first
       occurrence.

       -c, --count
              prefix lines by the number of occurrences

       -d, --repeated
              only print duplicate lines, one for each group

       -D     print all duplicate lines


       -i, --ignore-case
              ignore differences in case when comparing

Fields
       -f, --skip-fields=N
              avoid comparing the first N fields

Future
       -w, --check-chars=N
              compare no more than N characters in lines

       -s, --skip-chars=N
              avoid comparing the first N characters

       -u, --unique
              only print unique lines

Groups

       --group[=METHOD]
              show all items, separating groups with an empty line;
              METHOD={separate(default),prepend,append,both}

       --all-repeated[=METHOD]
              like -D, but allow separating groups with an empty line;
              METHOD={none(default),prepend,separate}

Assumes
- \n