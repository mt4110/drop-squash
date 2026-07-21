# Market Validation Tracker

Use this tracker outside the repository if it will contain personal data.
This file is the schema for collecting the four validation gates consistently.

## One Row Per Activated Tester

```text
activated_at:
user_id:
variant:
build_under_test:
first_session_date:
first_session_successful_files:
three_file_session: yes|no
second_day_with_success: yes|no
willing_to_pay: yes|no|unknown
price_range_note:
paid: yes|no
refund_requested: yes|no
stopped_reason:
notes:
```

## Rollup

Update these counts after every meaningful beta batch:

```text
activated_testers:
qualified_testers:
three_file_sessions_yes:
repeat_usage_yes:
willing_to_pay_yes:
paid_yes:
```

Also keep one short batch note:

```text
batch_review_date:
cohort_note:
top_variant_by_repeat_usage:
top_variant_by_three_file_completion:
top_variant_by_willingness_to_pay:
top_variant_by_paid_conversion:
single-biggest_blocker:
single-clearest_pull:
```

## Gate Formulas

- Repeat usage rate = `repeat_usage_yes / activated_testers`
- Three-file completion rate = `three_file_sessions_yes / qualified_testers`
- Willingness to pay rate = `willing_to_pay_yes / qualified_testers`
- Paid conversion rate = `paid_yes / qualified_testers`

## Gate Targets

- Repeat usage rate >= 40%
- Three-file completion rate >= 70%
- Willingness to pay rate >= 30%
- Paid conversion rate >= 10% or at least 5 paid testers

## Notes

- `qualified_testers` should mean testers who actually activated and attempted
  the core screen-recording job.
- Keep `variant` limited to `Work Screen Recordings`, `Private Media Library`,
  or `Creator Upload Preparation`.
- Record `stopped_reason` in plain words when the tester does not return.
- Treat reissues for the same tester as one user for gate math; do not inflate
  `activated_testers` or `paid_yes`.
- Update the batch note before editing
  [docs/market-decision-memo.md](/Users/masakitakemura/_workspace/drop-squash/docs/market-decision-memo.md).
